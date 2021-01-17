-- Create database for RCOS data
create database rcos;

-- Create schema for all tables served by Postgrest
create schema api;

-- Set the schema for future table/object creations
set schema 'api';
