import socket
import threading
import time
import struct
import sys

# Configuration
DEFAULT_APP_IP = "127.0.0.1"
STATUS_PORT = 1111  # App listens on this port
COMMAND_PORT = 8888 # App sends to this port

class SprayerSimulator:
    def __init__(self, target_ip):
        self.running = True
        self.target_pressure = 2.5
        self.current_pressure = 2.45
        self.speed = 8.5
        self.boom_locked = False
        self.target_ip = target_ip

        # Socket for sending status updates
        self.status_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        if self.target_ip == "255.255.255.255":
            self.status_sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
        
        # Socket for receiving commands/settings
        self.command_sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.command_sock.bind(("0.0.0.0", COMMAND_PORT))
        self.command_sock.settimeout(1.0)

    def start(self):
        print(f"Starting simulator...")
        print(f"Sending status to {self.target_ip}:{STATUS_PORT}")
        print(f"Listening for commands on port {COMMAND_PORT}")
        print("Commands:")
        print("  tp <val>  Set target pressure")
        print("  cp <val>  Set current pressure")
        print("  p <val>   Set both target and current pressure")
        print("  s <val>   Set speed")
        print("  b         Toggle boom lock")
        print("  q         Quit")
        
        # Start receiver thread
        self.receiver_thread = threading.Thread(target=self.receive_loop, daemon=True)
        self.receiver_thread.start()

        # Start input thread
        self.input_thread = threading.Thread(target=self.input_loop, daemon=True)
        self.input_thread.start()

        # Start sender loop
        try:
            while self.running:
                self.send_status()
                time.sleep(0.1) # 10Hz update rate
        except KeyboardInterrupt:
            self.stop()

    def input_loop(self):
        while self.running:
            try:
                cmd = input("> ").strip().lower()
                if not cmd:
                    continue
                
                parts = cmd.split()
                action = parts[0]
                
                if action == 'q':
                    self.running = False
                elif action == 'tp' and len(parts) > 1:
                    self.target_pressure = float(parts[1])
                    print(f"Target pressure set to {self.target_pressure}")
                elif action == 'cp' and len(parts) > 1:
                    self.current_pressure = float(parts[1])
                    print(f"Current pressure set to {self.current_pressure}")
                elif action == 'p' and len(parts) > 1:
                    self.target_pressure = float(parts[1])
                    self.current_pressure = self.target_pressure
                    print(f"Both pressures set to {self.target_pressure}")
                elif action == 's' and len(parts) > 1:
                    self.speed = float(parts[1])
                    print(f"Speed set to {self.speed}")
                elif action == 'b':
                    self.boom_locked = not self.boom_locked
                    print(f"Boom locked: {self.boom_locked}")
                else:
                    print("Unknown command. Use: p <val>, s <val>, b, q")
            except EOFError:
                self.running = False
                break
            except Exception as e:
                print(f"Input error: {e}")

    def stop(self):
        self.running = False
        # Threads are daemon, so they will exit when main thread exits or we can wait briefly
        print("Simulator stopping...")
        self.status_sock.close()
        self.command_sock.close()

    def send_status(self):
        # Header: 0x80, 0x81, 0x70, 0x70, 0x07
        header = bytes([0x80, 0x81, 0x70, 0x70, 0x07])
        
        target_p = int(self.target_pressure * 100)
        current_p = int(self.current_pressure * 100)
        speed = int(self.speed * 100)
        boom_locked = 1 if self.boom_locked else 0
        
        # Pack the data: < (little-endian), H (unsigned short), B (unsigned char)
        payload = struct.pack("<HHHB", target_p, current_p, speed, boom_locked)
        
        packet_without_crc = header + payload
        
        # CRC is sum of bytes from index 2 to 11 (inclusive)
        # In Python, packet_without_crc[2:] covers indices 2 to 11 (total 12 bytes - 2 = 10 bytes)
        crc = sum(packet_without_crc[2:]) & 0xFF
        
        packet = packet_without_crc + bytes([crc])
        
        try:
            self.status_sock.sendto(packet, (self.target_ip, STATUS_PORT))
        except Exception as e:
            print(f"Error sending status: {e}")

    def receive_loop(self):
        while self.running:
            try:
                data, addr = self.command_sock.recvfrom(1024)
                self.handle_command(data, addr)
            except socket.timeout:
                continue
            except Exception as e:
                if self.running:
                    print(f"Receiver error: {e}")

    def handle_command(self, data, addr):
        if len(data) < 5:
            return

        # Check header
        if data[0:2] != bytes([0x80, 0x81]):
            return

        # Check CRC
        if len(data) > 2:
            received_crc = data[-1]
            calculated_crc = sum(data[2:-1]) & 0xFF
            if received_crc != calculated_crc:
                print(f"CRC mismatch: received {received_crc}, calculated {calculated_crc}")
                return

        # Identify packet type
        packet_type = data[2:4]
        
        if packet_type == bytes([0x71, 0x70]): # Settings
            if len(data) != 16:
                print(f"Invalid settings packet length: {len(data)}")
                return
            
            nozzle_size = data[5] / 100.0
            nozzle_spacing = data[6] / 100.0
            litres_per_ha = struct.unpack("<H", data[7:9])[0] / 10.0
            min_pressure = struct.unpack("<H", data[9:11])[0] / 100.0
            max_pressure = struct.unpack("<H", data[11:13])[0] / 100.0
            nominal_pressure = struct.unpack("<H", data[13:15])[0] / 100.0
            
            print(f"Received Settings from {addr}:")
            print(f"  Nozzle Size: {nozzle_size}")
            print(f"  Nozzle Spacing: {nozzle_spacing}")
            print(f"  Rate: {litres_per_ha} L/ha")
            print(f"  Pressure Range: {min_pressure} - {max_pressure} bar")
            print(f"  Nominal Pressure: {nominal_pressure} bar")
            
        elif packet_type == bytes([0x71, 0x71]): # Button State
            if len(data) != 7:
                print(f"Invalid button state packet length: {len(data)}")
                return
            
            button_states = data[5]
            activated = (button_states & 0x01) != 0
            constant_pressure = (button_states & 0x02) != 0
            
            print(f"Received Button State from {addr}:")
            print(f"  Activated: {activated}")
            print(f"  Constant Pressure: {constant_pressure}")
        else:
            print(f"Unknown packet type {packet_type.hex()} from {addr}")

if __name__ == "__main__":
    target_ip = DEFAULT_APP_IP
    if len(sys.argv) > 1:
        target_ip = sys.argv[1]
    
    sim = SprayerSimulator(target_ip)
    sim.start()

