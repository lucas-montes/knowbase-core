#include "../include/db.h"
#include "../include/file_handler.h"
#include "../include/tfidf.h"
#include <iostream>
#include <vector>

int main(int argc, char *argv[])
{

  if (argc != 3)
  {
    std::cerr << "Usage: " << argv[0] << " <file_path>" << std::endl;
    return 1;
  }
  const char *filePath = argv[1];
  const char *action = argv[2];

  processFile(filePath);
  //  handleDb(filePath, action);
  return 0;
}
