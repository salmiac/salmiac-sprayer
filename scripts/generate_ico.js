const fs = require('fs');
const path = require('path');
const pngToIco = require('png-to-ico').default;

async function main() {
    try {
        const outDir = path.join(__dirname, '../assets');
        const buf = await pngToIco([
            path.join(outDir, 'logo_48.png'),
            path.join(outDir, 'logo_64.png'),
            path.join(outDir, 'logo_128.png'),
            path.join(outDir, 'logo_256.png')
        ]);
        
        fs.writeFileSync(path.join(outDir, 'icon.ico'), buf);
        console.log('Successfully generated icon.ico');
    } catch (error) {
        console.error('Error generating ICO:', error);
        process.exit(1);
    }
}

main();
