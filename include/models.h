#ifndef MODELS_H
#define MODELS_H

#include <string>

class File
{
public:
    const unsigned long int id;
    const std::string name;
    const std::string path;
};

class Word
{
public:
    const unsigned long int id;
    const std::string word;
};

class WordFileRelationship
{
public:
    const unsigned long int id;
    const std::string word_id;
    const std::string file_id;
    unsigned long int word_ocurrences;
};

#endif
