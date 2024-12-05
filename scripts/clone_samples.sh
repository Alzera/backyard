#!/bin/bash

TARGET_DIR="./target/samples"

if [ ! -d "$TARGET_DIR" ]; then
  mkdir "$TARGET_DIR"
fi

cd "$TARGET_DIR"

echo "Cloning Composer..."
git clone https://github.com/composer/composer.git composer

echo "Cloning Laravel..."
git clone https://github.com/laravel/laravel.git laravel

echo "Cloning Symfony..."
git clone https://github.com/symfony/symfony.git symfony

echo "Cloning WordPress..."
git clone https://github.com/WordPress/WordPress.git wordpress

echo "All repositories have been cloned into the '$TARGET_DIR' folder."
