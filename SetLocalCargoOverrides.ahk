Gui, Destroy
Gui, Add, Text, , Select folder(s):
Gui, Add, ListView, w180 r20 -Hdr gfolder_list, Name

Loop, %A_ScriptDir%\*, 2, 0
{
    if A_LoopFileAttrib contains H,R,S  ; Skip any file that is either H (Hidden), R (Read-only), or S (System).
        continue
    LV_Add("", A_LoopFileName)
}

Gui, Add, Button, default xm w80, OK
Gui, Add, Button, wp xp+100 yp, Cancel

GuiControl, Choose, folder_choice, 1
Gui, Show
Exit



folder_list:
if A_GuiEvent <> DoubleClick
    return


ButtonOK:
Gui, Submit
paths =
RowNumber = 0
Loop
{
    RowNumber := LV_GetNext(RowNumber)
    if not RowNumber
        break
    LV_GetText(folder_name, RowNumber)
    paths = %paths%"../%folder_name%"
    if LV_GetNext(RowNumber)
        paths .= ", "
}

if paths {
    FileCreateDir, .cargo
    FileDelete .cargo\config
    FileAppend, paths = [%paths%], .cargo\config
} else {
    FileRemoveDir, .cargo, 1
}


ButtonCancel:
GuiClose:
GuiEscape:
Gui, Destroy
ExitApp
