<?php

namespace OuterLoopExits;

function zeroBreak(bool $flag): void
{
    echo 'run';
    try {
        while ($flag) {
            break 0;
        }
    } finally {
        return;
    }
}

function zeroContinue(int $value): void
{
    echo 'run';
    try {
        switch ($value) {
            default:
                continue 0;
        }
    } finally {
        return;
    }
}

function localOneBreak(bool $flag): void
{
    echo 'run';
    try {
        while ($flag) {
            break 1;
        }
    } finally {
        return;
    }
}

function localOneContinue(bool $flag): void
{
    echo 'run';
    try {
        while ($flag) {
            continue;
        }
    } finally {
        return;
    }
}

function outerTwoBreak(bool $flag): void
{
    echo 'run';
    while ($flag) {
        try {
            while ($flag) {
                break 2;
            }
        } finally {
            return;
        }
    }
}

function outerThreeContinue(bool $flag): void
{
    echo 'run';
    while ($flag) {
        while ($flag) {
            try {
                while ($flag) {
                    continue 3;
                }
            } finally {
                return;
            }
        }
    }
}

function directOneBreak(bool $flag): void
{
    echo 'run';
    while ($flag) {
        try {
            break;
        } finally {
            return;
        }
    }
}

function directTwoContinue(bool $flag): void
{
    echo 'run';
    while ($flag) {
        while ($flag) {
            try {
                continue 2;
            } finally {
                return;
            }
        }
    }
}
