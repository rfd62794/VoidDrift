# gh_tools.ps1 — GitHub CLI helpers for VoidDrift project management
# Usage: . .\scripts\gh_tools.ps1

$REPO = "rfd62794/VoidDrift"
$PROJECT_NUMBER = 1  # Update if project number differs

function List-Issues {
    gh issue list --repo $REPO --state open
}

function View-Issue {
    param([int]$Number)
    gh issue view $Number --repo $REPO
}

function Close-Issue {
    param([int]$Number, [string]$Comment = "Complete.")
    gh issue close $Number --repo $REPO --comment $Comment
}

function Add-Issue {
    param(
        [string]$Title,
        [string]$Body,
        [string]$Label = ""
    )
    if ($Label) {
        gh issue create --repo $REPO --title $Title --body $Body --label $Label
    } else {
        gh issue create --repo $REPO --title $Title --body $Body
    }
}

function Update-Issue {
    param([int]$Number, [string]$Body)
    gh issue edit $Number --repo $REPO --body $Body
}

function Comment-Issue {
    param([int]$Number, [string]$Comment)
    gh issue comment $Number --repo $REPO --body $Comment
}

function List-ProjectItems {
    gh project item-list $PROJECT_NUMBER --owner rfd62794
}

Write-Host "gh_tools loaded. Commands: List-Issues, View-Issue, Close-Issue, Add-Issue, Update-Issue, Comment-Issue, List-ProjectItems" -ForegroundColor Cyan
