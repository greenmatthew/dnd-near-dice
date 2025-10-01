# Default recipe - shows all available commands
default: help

# Display all available recipes
@help:
    just --list

# Show all available .NET project templates (console, webapi, classlib, etc.)
@show-templates:
    dotnet new list

# Build the solution
@build:
    dotnet build

# Run all tests
@test:
    dotnet test

# Add a NuGet package to the project
@add-package PACKAGE:
    dotnet add package {{PACKAGE}}

# Restore dependencies
@restore:
    dotnet restore

# Clean build artifacts
@clean:
    dotnet clean