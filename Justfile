# Default recipe - shows all available commands
default: help

# Display all available recipes
@help:
    just --list

# Show all available .NET project templates (console, webapi, classlib, etc.)
@show-project-templates:
    dotnet new list

# Internal validation: Check if project file exists in current directory
_check-project:
    #!/usr/bin/env bash
    csproj_files=(*.csproj)
    if [ ! -e "${csproj_files[0]}" ]; then
        echo "Error: No project file found in current directory"
        exit 1
    elif [ ${#csproj_files[@]} -gt 1 ]; then
        echo "Error: Multiple project files found in current directory:"
        printf '  %s\n' "${csproj_files[@]}"
        echo "This tool expects exactly one .csproj file"
        exit 1
    fi

# Create a new C# script from template
[no-cd]
@new-script PATH NAMESPACE='': _check-project
    #!/usr/bin/env bash
    # Extract filename without extension for class name
    filename=$(basename "{{PATH}}")
    classname="${filename%.cs}"
    
    # Use provided namespace or derive from project/directory structure
    if [ -z "{{NAMESPACE}}" ]; then
        # Get project name from .csproj file
        csproj_file=(*.csproj)
        proj_name="${csproj_file[0]%.csproj}"
        
        # Get directory path relative to current directory
        dirpath=$(dirname "{{PATH}}")
        if [ "$dirpath" != "." ]; then
            namespace="${proj_name}.${dirpath//\//.}"
        else
            namespace="${proj_name}"
        fi
    else
        namespace="{{NAMESPACE}}"
    fi
    
    # Ensure directory exists
    mkdir -p "$(dirname "{{PATH}}")"

    echo 'Class name: '$classname
    echo 'Namespace: '$namespace
    echo 'PATH: {{PATH}}'
    
    # Create the script using custom template (specify template path directly)
    dotnet new class \        
        --output "$(dirname "{{PATH}}")" \
        --name "$classname" \ 
        --class-name "$classname" \
        --namespace "$namespace"

# Build the project
@build: _check-project
    dotnet build

# Run the project
@run: _check-project
    dotnet run

# Add a NuGet package to the project
@add-package PACKAGE:
    dotnet add package {{PACKAGE}}

# Restore dependencies
@restore: _check-project
    dotnet restore

# Clean build artifacts
@clean: _check-project
    dotnet clean