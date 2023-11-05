#ifndef TFIDF_H
#define TFIDF_H

#include <string>
#include <unordered_map>

class Tf
{
public:
    Tf(const std::string &filePath);
    ~Tf();

    std::unordered_map<std::string, unsigned long int> words;
    const std::string &filePath;
};

class Idf
{
public:
    Idf(const std::string &dbName);
    ~Idf();

    bool insertData(const std::string &data);
    void displayData();

private:
    std::string dbName;
};

#endif
