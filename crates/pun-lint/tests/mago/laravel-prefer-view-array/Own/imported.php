<?php

namespace App;

use function Illuminate\Support\view;
use function Other\make as view2;

view('a')->with('user', 1);
view2('a')->with('user', 1);
