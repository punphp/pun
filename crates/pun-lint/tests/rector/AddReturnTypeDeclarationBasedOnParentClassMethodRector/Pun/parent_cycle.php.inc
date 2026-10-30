<?php

namespace App;

class First extends Second
{
    public function run()
    {
    }
}

class Second extends First
{
    public function run()
    {
    }
}
