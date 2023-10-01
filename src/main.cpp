#include "../include/db.h"
#include <iostream>
#include <vector>

int handleDb(std::string filename, std::string action)
{
  Database db("/home/lucas/Projects/knowbase/test.db");

  if (!db.open())
  {
    return 1;
  }

  if (!db.createTable())
  {
    db.close();
    return 1;
  }

  if (action == "add")
  {

    if (!db.pathExists(filename))
    {
      db.insertData(filename);
    }
  }
  else if (action == "read")
  {
    db.displayData();
  }
  else if (action == "delete")
  {
    db.deleteData(filename);
  }
  else if (action == "update")
  {
    db.displayData();
  }
  else
  {
    std::cerr << "Invalid action: " << action << std::endl;
    return 1;
  }
  db.close();
  return 0;
}

int main(int argc, char *argv[])
{

  if (argc != 3)
  {
    std::cerr << "Usage: " << argv[0] << " <file_path>" << std::endl;
    return 1;
  }
  const char *filePath = argv[1];
  const char *action = argv[2];

  handleDb(filePath, action);
  return 0;
}
