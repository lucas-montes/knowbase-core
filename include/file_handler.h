#ifndef FILE_HANDLER_H
#define FILE_HANDLER_H

#include "./tfidf.h"
#include <string>
#include <unordered_map>

std::unordered_map<std::string, unsigned long int>
getWordCount(const std::string &filePath);

void processLine(
    const std::string &line,
    std::unordered_map<std::string, unsigned long int> &word_count);

void processWord(
    const std::string &word,
    std::unordered_map<std::string, unsigned long int> &word_count);

void saveWordCount(const std::string &filePath);

#endif
