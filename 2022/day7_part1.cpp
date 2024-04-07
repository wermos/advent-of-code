#include <fstream>
#include <cstdint>
#include <iostream>
#include <string>
#include <memory>
#include <unordered_map>

class fNode;

class File {
    public:
        File() : parent{nullptr}, name{}, size{0} {}

        File(std::string name, std::shared_ptr<fNode> parent) : parent{parent}, name{name}, size{0} {}

        void updateSize(std::uint64_t new_size) {
            size = new_size;
        }

    private:
        std::shared_ptr<fNode> parent;

        std::string name;
        std::uint64_t size;
};

class fNode {
    public:
        fNode() : parent{nullptr}, subdirectories{}, files{}, name{}, size{0} {}

        fNode(std::string name, std::shared_ptr<fNode> parent) : parent{parent}, subdirectories{}, files{}, name{name}, size{0} {}

        void addFile(std::string name, bool isFile) {
            files[name] = std::make_shared<File>(name, this);
        }

        void addSubdirectory(std::string name, bool isFile) {
            subdirectories[name] = std::make_shared<fNode>(name, this);
        }

        void updateSize(std::string name, bool isFile, std::uint64_t size) {
            if (isFile) {
                files[name]->updateSize(size);
            } else {
                subdirectories[name]->updateSize(size);
            }
        }

        void updateSize(std::uint64_t new_size) {
            size = new_size;
        }

    // private:
        std::shared_ptr<fNode> parent;
        std::unordered_map<std::string, std::shared_ptr<fNode>> subdirectories;
        std::unordered_map<std::string, std::shared_ptr<File>> files;

        std::string name;
        std::uint64_t size;
};

class FileSystem {
    public:
        FileSystem() = default;

        void initialize() {
            root = std::make_shared<fNode>();

            current = root;
        }

        void cd(std::string name) {
            current = root->subdirectories[name];
        }

        void update() {
            
        }

    private:
        std::shared_ptr<fNode> root;
        std::weak_ptr<fNode> current;
};

int main() {
    std::ifstream input{"inputs/day7-test.txt"};

    FileSystem fs;
    std::string line;

    // bool ls_output = false;

    while (std::getline(input, line)) {
        if (line[0] == '$') {
            // then a command was entered
            if (line.substr(2, 4) == "cd") {
                std::string dir_name = line.substr(5);

                if (dir_name == "/") {
                    fs.initialize();
                } else {

                }

            } else if (line.substr(2, 4) == "ls") {
                // ls_output = true;
            }
        } else {

        }
    }
}