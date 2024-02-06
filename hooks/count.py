import os
import re
import subprocess
import math

def is_file_tracked(file_path):
    try:
        # Check if the file is tracked by Git
        subprocess.check_output(['git', 'ls-files', '--error-unmatch', file_path], stderr=subprocess.STDOUT)
        return True
    except subprocess.CalledProcessError:
        return False

def count_files(directory, valid_extensions):
    file_pattern = re.compile(r'day(\d+)_part([12])\.\w+')
    file_count = {'1': 0, '2': 0}

    for root, _, filenames in os.walk(directory):
        for filename in filenames:
            match = file_pattern.match(filename)
            if match:
                day, part = match.groups()
                file_path = os.path.join(root, filename)

                if is_file_tracked(file_path) and any(filename.lower().endswith(ext) for ext in valid_extensions):
                    file_count[part] += 1

    return file_count

def calculate_progress(counts):
    total_files = sum(counts.values())
    progress_percentage = total_files * 2
    return min(progress_percentage, 100)  # Cap the progress at 100%

def calculate_overall_progress(year_counts, total_years):
    expected_files = 25 * 2 * total_years
    # number of problems per year, times the number of parts, time the number of years

    # print([data for data in year_counts.values()])
    actual_files = sum([data for data in year_counts.values()])
    overall_progress_percentage = min(math.floor((actual_files / expected_files) * 100), 100)
    return overall_progress_percentage

def count_files_in_years(root_directory, valid_extensions):
    year_pattern = re.compile(r'^\d{4}$')
    year_progress = {}

    for subdir in os.listdir(root_directory):
        year_path = os.path.join(root_directory, subdir)

        if os.path.isdir(year_path) and year_pattern.match(subdir):
            counts = count_files(year_path, valid_extensions)
            progress = calculate_progress(counts)
            year_progress[subdir] = progress
            # print(f"Year {subdir}: Part 1 - {counts['1']}, Part 2 - {counts['2']}, Progress - {progress}%")

    return year_progress

def update_readme_with_progress(readme_path, year_progress, overall_progress):
    with open(readme_path, 'r') as file:
        readme_content = file.read()

    # Update overall progress
    overall_pattern = re.compile(r'!\[Progress Bar to show how much progress has been made\]\(https://progress-bar.dev/\d+/\?title=Progress\)')
    overall_content = f"![Progress Bar to show how much progress has been made](https://progress-bar.dev/{overall_progress}/?title=Progress)"
    readme_content = overall_pattern.sub(overall_content, readme_content)

    # Update progress for each year
    for year, progress in year_progress.items():
        year_pattern = re.compile(fr'!\[Progress Bar to show how much progress has been made in the {year} problems\]\(https://progress-bar.dev/\d+/\?title={year}\)')
        year_content = f"![Progress Bar to show how much progress has been made in the {year} problems](https://progress-bar.dev/{progress}/?title={year})"
        readme_content = year_pattern.sub(year_content, readme_content)

    with open(readme_path, 'w') as file:
        file.write(readme_content)

    with open(readme_path, 'w') as file:
        file.write(readme_content)

def commit_readme_changes(commit_message, readme_path):
    os.system(f"git add {readme_path}")
    os.system(f"git commit -m \"{commit_message}\"")
    # print(f"git commit -m \"{commit_message}\"")

def run_clang_format(directory, file_extensions, target_directories):
    formatted_files = []
    for root, _, filenames in os.walk(directory):
        if root.endswith(tuple(target_directories)):
            for filename in filenames:
                if filename.endswith(tuple(file_extensions)):
                    file_path = os.path.join(root, filename)
                    if is_file_tracked(file_path):
                        os.system(f"clang-format --style=file -i {file_path}")
                        # print(f"clang-format --style=file -i {file_path}")
                        formatted_files.append(file_path)

    return formatted_files

def commit_formatted_files(formatted_files, commit_message):
    if formatted_files:
        formatted_files_str = ' '.join(formatted_files)
        os.system(f"git add {formatted_files_str}")
        os.system(f"git commit -m \"{commit_message}\"")
        # print(f"git add {formatted_files_str}")
        # print(f"git commit -m \"{commit_message}\"")

def main():
    root_directory = "."
    valid_extensions = ['.cpp', '.java', '.py', '.rs']

    year_progress = count_files_in_years(root_directory, valid_extensions)
    total_years = len(year_progress)

    # Run clang-format on .cpp and .hpp files
    target_directories = ['2022', '2023/src']
    formatted_files = run_clang_format(root_directory, ['.cpp', '.hpp'], target_directories)
    commit_formatted_files(formatted_files, r"Formatted the files using \`clang-format\`.")

    # Calculate and print the overall progress
    overall_progress = calculate_overall_progress(year_progress, total_years)
    # print(f"Overall Progress: {overall_progress}%")

    # Update README with progress and commit changes
    readme_path = "./README.md"  # Update with the actual path to your README
    update_readme_with_progress(readme_path, year_progress, overall_progress)
    commit_message = "Updated README to reflect current progress using pre-push git hook."
    commit_readme_changes(commit_message, readme_path)

if __name__ == "__main__":
    main()
