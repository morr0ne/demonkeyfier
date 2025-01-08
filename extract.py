import os.path
import sys
import subprocess

ISOS = [
    "SLUS-20685",  # USA
    "SCCS-40001",  # China(?)
    "SCES-50885",  # Europe
    "SCES-51102",  # France
    "SCES-51104",  # Germany
    "SCES-51103",  # Italy
    "SCES-51105",  # Spain
    "SCPS-15025",  # Original Japanese release
]

for iso in ISOS:
    if not os.path.isfile(f"isos/{iso}.iso"):
        sys.stderr.write(f"Missing iso for {iso}")
        sys.exit(1)

for iso in ISOS:
    os.makedirs(f"extracted/{iso}", exist_ok=True)
    subprocess.run(
        ["bsdtar", "-xpkvf", f"isos/{iso}.iso", "-C", f"extracted/{iso}"],
        check=True,
    )
