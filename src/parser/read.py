import time
import re
import argparse
from loguru import logger


# APPLICATION PARAMETERS ----------------------------

# Set up argument parser
parser = argparse.ArgumentParser(description="Process a CK3 save file.")
parser.add_argument(
    "--filename",
    nargs="?",
    default="data/latest/gamestate.ck3",
    help="Name of the readable CK3 save file",
)


# Parse arguments
args = parser.parse_args()

# Use the filename from arguments
filename = args.filename

logger.info(f"Using save file: {filename}")



# FUNCTION ----------------------------

def import_file(filename: str) -> str:
    """Import a CK3 save file."""

    start_time = time.time()

    with open(filename, "r", encoding="utf-8") as myfile:
        data = myfile.read()

        # Calculate file length and number of lines
        file_length = len(data)
        line_count = len(data.split("\n"))

        # Log formatted output with spaces as thousand separators
        logger.info(
            f"File length: {file_length:,}".replace(",", " ")
            + f" characters ({line_count:,}".replace(",", " ")
            + " lines)"
        )

    end_time = time.time()
    reading_time = end_time - start_time

    # Log the reading time
    logger.info(f"Reading time: {reading_time:.2f} seconds")

    return data


# FUNCTION ----------------------------

data = import_file(filename)
charachterhistory = re.findall(r'played_character={.+?\n}', data, re.S)