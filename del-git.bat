@echo off
FOR /D /r %%d in (*) DO (
    if exist "%%d\.git" (
        rd /s /q "%%d\.git"
        echo Deleted: %%d\.git
    )
)