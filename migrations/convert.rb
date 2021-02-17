#!/usr/bin/env ruby

require 'fileutils'

files = Dir.glob("*.sql")

for file in files do
  puts file
  f = File.read file
  dirname = "./" + File.basename(file, ".sql")
  up, down = f.delete_prefix("-- migrate:up").split("-- migrate:down")

  FileUtils.rm_rf dirname
  Dir.mkdir dirname
  IO.write(dirname + "/up.sql", up.strip)
  IO.write(dirname + "/down.sql", down.strip)
end
