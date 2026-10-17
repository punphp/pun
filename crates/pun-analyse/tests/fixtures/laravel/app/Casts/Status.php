<?php

declare(strict_types=1);

namespace App\Casts;

enum Status: string
{
    case Draft = 'draft';
}
