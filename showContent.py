import os

def print_file_contents(start_path, output_file):
    with open(output_file, 'w', encoding='utf-8') as out:
        for root, dirs, files in os.walk(start_path):
            for file in files:
                file_path = os.path.join(root, file)
                try:
                    out.write(f"File: {file_path}\n")  # Write the file name to output file
                    with open(file_path, 'r', encoding='utf-8') as f:
                        out.write(f.read())  # Write the content of the file to output file
                except Exception as e:
                    out.write(f"Could not read file {file_path}. Reason: {e}\n")
                out.write("\n" + "#" * 80 + "\n\n")  # Separator between files

# Replace 'your_project_directory_path' with the path to your project directory
start_path = './src'
output_file = 'project_contents.txt'  # Output file where the results will be saved
print_file_contents(start_path, output_file)
