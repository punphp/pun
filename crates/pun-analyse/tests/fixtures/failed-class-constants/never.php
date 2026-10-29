<?php

namespace FailedClassConstantNever;

final class Known { const C = 1; }
function stop(): never { exit(); }

function selectorOperand(): void
{
    $_result = Known::{stop()};
    echo 'after-never-selector';
}

function directOperand(): void
{
    stop();
    echo 'after-never-direct';
}
