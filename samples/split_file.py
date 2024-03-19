import sys

def split_file_into_separate_json_files(source_file):
    # Extract the base file name without extension to append _<linenumber>.json
    base_name = source_file.rsplit('.', 1)[0]
    
    with open(source_file, 'r') as file:
        for index, line in enumerate(file, start=1):
            # Create a new file name for each line, e.g., filename_1.json, filename_2.json, etc.
            new_file_name = f"{base_name}_{index}.json"
            
            # Open a new file for writing and write the line to it.
            with open(new_file_name, 'w') as new_file:
                new_file.write(line)
                
            # Optional: Print the name of the file that was created, for confirmation.
            print(f"Created {new_file_name}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python script.py <source_file_path>")
    else:
        source_file_path = sys.argv[1]
        split_file_into_separate_json_files(source_file_path)

