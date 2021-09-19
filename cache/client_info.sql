/*
 Navicat Premium Data Transfer

 Source Server         : sqlite
 Source Server Type    : SQLite
 Source Server Version : 3017000
 Source Schema         : main

 Target Server Type    : SQLite
 Target Server Version : 3017000
 File Encoding         : 65001

 Date: 10/09/2021 16:50:25
*/

PRAGMA foreign_keys = false;

-- ----------------------------
-- Table structure for client_info
-- ----------------------------
DROP TABLE IF EXISTS "client_info";
CREATE TABLE "client_info" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "cli_name" TEXT NOT NULL,
  "cli_key" TEXT NOT NULL,
  "cli_trs_type" TEXT NOT NULL,
  "cli_trs_host" TEXT NOT NULL,
  "cli_trs_port" integer NOT NULL
);

-- ----------------------------
-- Auto increment value for client_info
-- ----------------------------

PRAGMA foreign_keys = true;
