from pathlib import Path
import tomllib
import platform

DIR_HOME = Path.home()

if platform.system() == "Darwin":
    DIR_ZED_ROOT = Path(DIR_HOME / "Library/Application Support/Zed")
elif platform.system() == "Windows":
    raise Exception("Windows not supported.")
    # DIR_ZED_ROOT = Path(DIR_HOME / "AppData/Roaming/Zed")
else:
    raise Exception("OS not supported.")

DIR_EXTENSIONS = Path(DIR_ZED_ROOT / "extensions")
DIR_INSTALLED = Path(DIR_EXTENSIONS / "installed")


def validate_dirs():
    if not DIR_ZED_ROOT.exists():
        print("No zed found")
        return

    if not DIR_EXTENSIONS.exists():
        print("No extensions found")
        return

    if not DIR_INSTALLED.exists():
        print("No installed extensions found")
        return


def main():
    validate_dirs()

    print("Zed directory:", DIR_ZED_ROOT)
    print()

    for extension in DIR_INSTALLED.iterdir():
        extension_toml = Path(extension / "extension.toml")
        if not extension_toml.exists():
            # print(f"No extension.toml found: {extension.name}")
            continue

        # Read a TOML file
        with open(extension_toml, "rb") as f:
            data = tomllib.load(f)
            repo = data["repository"]
            print(f"{extension.name} - {repo}")
        f.close()


if __name__ == "__main__":
    main()
