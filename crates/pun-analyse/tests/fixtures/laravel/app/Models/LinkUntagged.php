<?php

declare(strict_types=1);

namespace App\Models;

use Illuminate\Database\Eloquent\Model;

/** Pinkary's Link without its @property tags, so every attribute comes from the migrations. */
final class LinkUntagged extends Model
{
    protected $table = 'links';
}
