@echo OFF

rem Handle no args, or help arg being passed
if [%1]==[] (
    call :show_help
    exit /b 0
)
echo(%*|findstr /irc:"\<-h\>" /c:"\</?\>" /c:"\<--help\>" >nul && (
    call :show_help
    exit /b 0
)




rustc --version
set "starting_path=%cd%"
set super_project_path=%~dp0

for /d %%g in ("%super_project_path%*") do (
    echo Running '%*' in %%g
    cd %%g
    %*
    echo ================================================================================
)

cd %starting_path%
goto :eof




:show_help
setlocal EnableDelayedExpansion
set output=Pass args to be invoked in each repository.^

^

Example usage:^

    %~nx0 cargo test --release
echo !output!
goto :eof
