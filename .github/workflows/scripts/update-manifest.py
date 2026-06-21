import sys
import re

def update_manifest(filepath, tag, commit):
    with open(filepath, 'r') as f:
        content = f.read()

    # Locate the source block for cosmic-utils-enroll git repository:
    # - type: git
    #   url: https://github.com/cosmic-utils/enroll.git
    #   tag: ...
    #   commit: ...
    pattern = r"(-\s*type:\s*git\s*\n\s*url:\s*https://github\.com/cosmic-utils/enroll\.git\s*\n\s*tag:\s*)([^\s\n]+)(\s*\n\s*commit:\s*)([^\s\n]+)"
    
    def repl(match):
        return f"{match.group(1)}{tag}{match.group(3)}{commit}"
        
    new_content, count = re.subn(pattern, repl, content)
    if count == 0:
        # Fallback line-by-line replacement if format slightly changes
        print("Regex pattern match failed, trying fallback line-by-line replacement...", file=sys.stderr)
        lines = content.splitlines()
        url_idx = -1
        for i, line in enumerate(lines):
            if "url: https://github.com/cosmic-utils/enroll.git" in line:
                url_idx = i
                break
        if url_idx != -1:
            updated = False
            for offset in [1, 2]:
                if url_idx + offset < len(lines):
                    line = lines[url_idx + offset]
                    if line.strip().startswith("tag:"):
                        indent = len(line) - len(line.lstrip())
                        lines[url_idx + offset] = " " * indent + f"tag: {tag}"
                        updated = True
            for offset in [1, 2]:
                if url_idx + offset < len(lines):
                    line = lines[url_idx + offset]
                    if line.strip().startswith("commit:"):
                        indent = len(line) - len(line.lstrip())
                        lines[url_idx + offset] = " " * indent + f"commit: {commit}"
                        updated = True
            if updated:
                new_content = "\n".join(lines) + "\n"
                count = 1
                
    if count > 0:
        with open(filepath, 'w') as f:
            f.write(new_content)
        print(f"Updated manifest file {filepath} to tag '{tag}' and commit '{commit}' successfully!")
    else:
        print("Error: Could not find/update the git source in manifest!", file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    if len(sys.argv) < 4:
        print("Usage: python3 update-manifest.py <manifest_file> <tag> <commit>", file=sys.stderr)
        sys.exit(1)
    update_manifest(sys.argv[1], sys.argv[2], sys.argv[3])
