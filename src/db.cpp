#include "../include/db.h"
#include <iostream>
#include <sqlite3.h>

Database::Database(const std::string &dbName) : db(nullptr), dbName(dbName) {}

Database::~Database()
{
  if (db)
  {
    sqlite3_close(db);
  }
}

bool Database::open()
{
  int rc = sqlite3_open(dbName.c_str(), &db);
  if (rc)
  {
    std::cerr << "Error opening database: " << sqlite3_errmsg(db) << std::endl;
    return false;
  }
  return true;
}

bool Database::close()
{
  if (db)
  {
    sqlite3_close(db);
    db = nullptr;
  }
  return true;
}

bool Database::pathExists(const std::string &filepath)
{
  const char *sql = "SELECT COUNT(*) FROM files WHERE filepath = ?;";

  sqlite3_stmt *stmt;
  if (sqlite3_prepare_v2(db, sql, -1, &stmt, nullptr) != SQLITE_OK)
  {
    std::cerr << "Error preparing the SQL statement" << std::endl;
    sqlite3_close(db);
    return 1;
  }

  // Bind the pathToCheck to the query parameter
  if (sqlite3_bind_text(stmt, 1, filepath.c_str(), filepath.length(),
                        SQLITE_STATIC) != SQLITE_OK)
  {
    std::cerr << "Error binding the parameter" << std::endl;
    sqlite3_finalize(stmt);
    sqlite3_close(db);
    return 1;
  }

  int result = 0;
  if (sqlite3_step(stmt) == SQLITE_ROW)
  {
    result = sqlite3_column_int(stmt, 0);
  }

  sqlite3_finalize(stmt);

  return result > 0;
}

bool Database::createTable()
{
  const char *createTableSQL = "CREATE TABLE IF NOT EXISTS files (id INTEGER "
                               "PRIMARY KEY, filepath TEXT);";
  char *errMsg = nullptr;
  int rc = sqlite3_exec(db, createTableSQL, nullptr, nullptr, &errMsg);

  if (rc != SQLITE_OK)
  {
    std::cerr << "SQL error: " << errMsg << std::endl;
    sqlite3_free(errMsg);
    return false;
  }
  return true;
}

bool Database::

    insertData(const std::string &data)
{
  const char *insertSQL = "INSERT INTO files (filepath) VALUES (?);";
  sqlite3_stmt *stmt;

  int rc = sqlite3_prepare_v2(db, insertSQL, -1, &stmt, nullptr);
  if (rc != SQLITE_OK)
  {
    std::cerr << "SQL error: " << sqlite3_errmsg(db) << std::endl;
    return false;
  }

  sqlite3_bind_text(stmt, 1, data.c_str(), data.length(), SQLITE_STATIC);
  rc = sqlite3_step(stmt);
  sqlite3_finalize(stmt);

  if (rc != SQLITE_DONE)
  {
    std::cerr << "SQL error: " << sqlite3_errmsg(db) << std::endl;
    return false;
  }
  return true;
}

bool Database::updateData(const std::string &newData)
{
  const char *updateSQL = "UPDATE files SET filepath = ? WHERE filepath = ?;";
  sqlite3_stmt *stmt;

  int rc = sqlite3_prepare_v2(db, updateSQL, -1, &stmt, nullptr);
  if (rc != SQLITE_OK)
  {
    std::cerr << "SQL error: " << sqlite3_errmsg(db) << std::endl;
    return false;
  }

  sqlite3_bind_text(stmt, 1, newData.c_str(), newData.length(), SQLITE_STATIC);
  sqlite3_bind_text(stmt, 2, newData.c_str(), newData.length(), SQLITE_STATIC);

  rc = sqlite3_step(stmt);
  sqlite3_finalize(stmt);

  if (rc != SQLITE_DONE)
  {
    std::cerr << "SQL error: " << sqlite3_errmsg(db) << std::endl;
    return false;
  }
  return true;
}

bool Database::deleteData(const std::string &filepath)
{
  const char *deleteSQL = "DELETE FROM files WHERE filepath = ?;";
  sqlite3_stmt *stmt;

  int rc = sqlite3_prepare_v2(db, deleteSQL, -1, &stmt, nullptr);
  if (rc != SQLITE_OK)
  {
    std::cerr << "SQL error: " << sqlite3_errmsg(db) << std::endl;
    return false;
  }

  sqlite3_bind_text(stmt, 1, filepath.c_str(), filepath.length(),
                    SQLITE_STATIC);

  rc = sqlite3_step(stmt);
  sqlite3_finalize(stmt);

  if (rc != SQLITE_DONE)
  {
    std::cerr << "SQL error: " << sqlite3_errmsg(db) << std::endl;
    return false;
  }
  return true;
}

void Database::displayData()
{
  const char *selectSQL = "SELECT id, filepath FROM files;";
  sqlite3_stmt *stmt;

  int rc = sqlite3_prepare_v2(db, selectSQL, -1, &stmt, nullptr);
  if (rc != SQLITE_OK)
  {
    std::cerr << "SQL error: " << sqlite3_errmsg(db) << std::endl;
    return;
  }

  std::cout << "ID\\filePath" << std::endl;
  while (sqlite3_step(stmt) == SQLITE_ROW)
  {
    int id = sqlite3_column_int(stmt, 0);
    const char *content =
        reinterpret_cast<const char *>(sqlite3_column_text(stmt, 1));
    std::cout << id << "\t" << content << std::endl;
  }

  sqlite3_finalize(stmt);
}

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
