# Toy File System
A small file system.

# Table Of Contents
- [Overview](#overview)
- [Design](#design)

# Overview
A small file system implemented in Rust for the purpose of learning Rust.

Simple with lots of constraints. 

# Design
## Storage Medium
The file system will use one file as a "disk". 

## Super Block
0th block in file system, stores metadata:

- Version of file system
- Inode usage bitmap

## Inodes
- Name
- Size
- Directory?
- Block pointers
