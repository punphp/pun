<?php

namespace Probe;

function f()
{
    $assert = function ($users) {
        $assertions = [
            ['first_name' => 'Thomas'],
            ['first_name' => 'Agent'],
        ];

        foreach ($assertions as $key => $assertion) {
            echo $users[$key]->only('first_name') === $assertion;
        }
    };
}
