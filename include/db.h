#ifndef DB_H
#define DB_H

#include <sqlite3.h>

#include <string>

class Database
{
public:
    Database(const std::string &dbName);
    ~Database();

    bool open();
    bool close();
    bool createTable();
    bool insertData(const std::string &data);
    bool updateData(const std::string &newData);
    bool deleteData(const std::string &data);
    bool pathExists(const std::string &data);
    void displayData();

private:
    sqlite3 *db;
    std::string dbName;
};

class Model
{
public:
    virtual std::string insert();

private:
    virtual std::string table();
};
#endif
