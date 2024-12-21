import zipfile
import argparse
from pathlib import Path
from loguru import logger

# Set up argument parser
parser = argparse.ArgumentParser(description='Unzip a CK3 save file and rename the extracted files.')
parser.add_argument('--ck3_save_file', default=None, help='Path to the CK3 save file')
parser.add_argument('--extract_dir', default='data/latest', help='Directory to extract the CK3 save file')

# Parse arguments
args = parser.parse_args()

# EXTRACT FILENAME ----------------------------

if args.ck3_save_file:
    zip_file_path = Path(args.ck3_save_file)
else:
    data_dir = Path('data')
    zip_file_path = max(data_dir.glob('*.ck3'), key=lambda p: p.stat().st_mtime)

logger.info(f'Using most recent CK3 save file: {zip_file_path}')

# Set the extraction directory
extract_dir = Path(args.extract_dir)

# Create the extraction directory if it doesn't exist
extract_dir.mkdir(parents=True, exist_ok=True)

# UNZIP ----------------------------

# Remove the extraction directory if it exists
if extract_dir.exists():
    for item in extract_dir.iterdir():
        if item.is_dir():
            item.rmdir()
        else:
            item.unlink()
    extract_dir.rmdir()

# Unzip the file
with zipfile.ZipFile(zip_file_path, 'r') as zip_ref:
    zip_ref.extractall(extract_dir)

    # Rename the 'gamestate' file to 'gamestate.ck3'
    gamestate_file = extract_dir / 'gamestate'
    if gamestate_file.exists():
        gamestate_file.rename(extract_dir / 'gamestate.ck3')

logger.info(f'Unzipped {zip_file_path} to {extract_dir / "gamestate.ck3"} and renamed files to have .ck3 extension')