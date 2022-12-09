from typing import *


class Directory:
    def __init__(self, _name: str = "", _size: int = 0, _parent=None, _files: Dict = None, _directories: Dict = None):
        self.name = _name
        self.size = _size
        self.parent = _parent
        self.files = _files or {}
        self.directories = _directories or {}

    def __str__(self):
        return f'name: {self.name} \nsize: {self.size}, \nparent: {self.parent and self.parent.name or None} \nfiles: {[(file, size) for file, size in self.files.items()]} \ndirectories: {[d for d in self.directories.keys()]}'

    def getChildDirectory(self, name):
        return self.directories[name]

    def getParentDirectory(self):
        return self.parent

    def sumFiles(self):
        for size in self.files.values():
            self.size += size

    def sumDirectories(self):
        for directory in self.directories.values():
            self.size += directory.size


with open("day7/input.txt") as file:
    lineTokens = [line.strip().split() for line in file.readlines()]


def constructFileSystem():
    curNode = root = Directory(_directories={"/": Directory("/")})
    for tokens in lineTokens:
        if (tokens[0] == "$"):
            command = tokens[1]
            if (command == "cd"):
                directory = tokens[2]
                if (directory == ".."):
                    curNode = curNode.getParentDirectory()
                else:
                    curNode = curNode.getChildDirectory(directory)
            else:
                continue
        elif (tokens[0] == "dir"):
            directory = tokens[1]
            curNode.directories[directory] = Directory(
                directory, 0, curNode, {}, {})
        else:
            size, file = tokens
            curNode.files[file] = eval(size)

    root = root.getChildDirectory("/")
    updateFileSystemSize(root)
    return root


def printFileSystem(node: Directory):
    print(node, end="\n\n")
    for directory in node.directories.values():
        printFileSystem(directory)


def updateFileSystemSize(node: Directory):
    node.sumFiles()
    for directory in node.directories.values():
        updateFileSystemSize(directory)
    node.sumDirectories()


def partOne(root: Directory):
    total = 0

    def dfs(node: Directory):
        nonlocal total
        if (node.size <= 100_000):
            total += node.size
        for directory in node.directories.values():
            dfs(directory)

    dfs(root)
    return total


def partTwo(root: Directory):
    minimumRemoved = float('inf')
    unusedSpace = 70_000_000 - root.size
    neededSpace = 30_000_000

    def dfs(node: Directory):
        nonlocal minimumRemoved
        unusedSpaceAfterRemoval = unusedSpace + node.size
        if (unusedSpaceAfterRemoval >= neededSpace and node.size <= minimumRemoved):
            minimumRemoved = node.size
        for directory in node.directories.values():
            dfs(directory)

    dfs(root)
    return minimumRemoved


root = constructFileSystem()
print(
    f'Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?: {partOne(root)}')

print(
    f'Find the smallest directory that, if deleted, would free up enough space on the filesystem to run the update. What is the total size of that directory?: {partTwo(root)}')
