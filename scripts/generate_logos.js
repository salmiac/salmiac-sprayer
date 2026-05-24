const fs = require('fs');
const path = require('path');

const sharp = require('sharp');

const sizes = [48, 64, 72, 96, 128, 144, 192, 256, 512];
const svgPath = path.join(__dirname, '../assets/logo.svg');
const outDir = path.join(__dirname, '../assets');

async function main() {
    try {
        const svgBuffer = fs.readFileSync(svgPath);

        for (const size of sizes) {
            const outPath = size === 512 
                ? path.join(outDir, `logo.png`) 
                : path.join(outDir, `logo_${size}.png`);
            
            await sharp(svgBuffer)
                .resize(size, size)
                .png()
                .toFile(outPath);
            console.log(`Generated ${outPath}`);
        }
        
        // Also explicitly generate logo_512.png to be sure
        await sharp(svgBuffer)
            .resize(512, 512)
            .png()
            .toFile(path.join(outDir, 'logo_512.png'));
        console.log(`Generated logo_512.png`);

        console.log('All PNGs generated successfully.');
    } catch (error) {
        console.error('Error generating PNGs:', error);
        process.exit(1);
    }
}

main();
