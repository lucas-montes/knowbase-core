#include "../include/file_handler.h"
#include "../include/db.h"
#include <fstream>
#include <iostream>
#include <ostream>
#include <sstream>
#include <string>
#include <vector>

std::unordered_map<std::string, unsigned long int>
getWordCount(const std::string &filePath)
{
  std::ifstream file(filePath);

  std::unordered_map<std::string, unsigned long int> word_count;
  if (!file.is_open())
  {
    std::cerr << "Failed to open the file." << std::endl;
    return word_count;
  }
  std::string line;
  // Read the file line by line
  while (std::getline(file, line))
  {
    processLine(line, word_count);
  }
  // Close the file
  file.close();

  return word_count;
}

void processLine(
    const std::string &line,
    std::unordered_map<std::string, unsigned long int> &word_count)
{
  // Create a string stream to split the line into words
  std::istringstream iss(line);
  std::string word;
  std::vector<std::string> words;

  // Split the line into words
  while (iss >> word)
  {
    words.push_back(word);
  }

  // Process the words in the current line
  for (std::string &w : words)
  {
    processWord(w, word_count);
  }
}

void processWord(
    std::string &word,
    std::unordered_map<std::string, unsigned long int> &word_count)
{
  word.erase(std::remove_if(word.begin(), word.end(), ispunct), word.end());
  word_count[word] += 1;
}
