<?php

declare(strict_types=1);

namespace App;

use App\Models\Child;
use App\Models\Everything;
use App\Models\GlobalAlias;
use App\Models\Link;
use App\Models\LinkUntagged;
use App\Models\NoTable;
use App\Models\PollVoteUntagged;
use App\Models\Third;

/** The model attributes Larastan types, one dumpType() per line; `scripts/export-laravel-fixture.py` runs PHPStan on it. */
function probe(
    Link $l,
    LinkUntagged $u,
    PollVoteUntagged $v,
    Everything $e,
    Child $c,
    NoTable $n,
    Third $t,
    GlobalAlias $g,
    ?LinkUntagged $mu,
    LinkUntagged|PollVoteUntagged $either,
    \stdClass $std,
    mixed $untyped,
    string $name,
): void {
    \PHPStan\dumpType($l->is_visible);
    \PHPStan\dumpType($l->click_count);
    \PHPStan\dumpType($l->url);
    \PHPStan\dumpType($u->id);
    \PHPStan\dumpType($u->description);
    \PHPStan\dumpType($u->url);
    \PHPStan\dumpType($u->user_id);
    \PHPStan\dumpType($u->click_count);
    \PHPStan\dumpType($u->is_visible);
    \PHPStan\dumpType($u->created_at);
    \PHPStan\dumpType($u->updated_at);
    \PHPStan\dumpType($v->id);
    \PHPStan\dumpType($v->user_id);
    \PHPStan\dumpType($v->poll_option_id);
    \PHPStan\dumpType($v->created_at);
    \PHPStan\dumpType($e->id);
    \PHPStan\dumpType($e->int_col);
    \PHPStan\dumpType($e->unsigned_int_col);
    \PHPStan\dumpType($e->uint_col);
    \PHPStan\dumpType($e->nullable_false);
    \PHPStan\dumpType($e->nullable_const);
    \PHPStan\dumpType($e->nullable_one);
    \PHPStan\dumpType($e->nullable_upper);
    \PHPStan\dumpType($e->nullable_named);
    \PHPStan\dumpType($e->text_col);
    \PHPStan\dumpType($e->bool_col);
    \PHPStan\dumpType($e->point_col);
    \PHPStan\dumpType($e->decimal_col);
    \PHPStan\dumpType($e->float_col);
    \PHPStan\dumpType($e->enum_col);
    \PHPStan\dumpType($e->enum_empty);
    \PHPStan\dumpType($e->status);
    \PHPStan\dumpType($e->set_col);
    \PHPStan\dumpType($e->json_col);
    \PHPStan\dumpType($e->unknown_col);
    \PHPStan\dumpType($e->added_col);
    \PHPStan\dumpType($e->taggable_type);
    \PHPStan\dumpType($e->taggable_id);
    \PHPStan\dumpType($e->nullable_taggable_type);
    \PHPStan\dumpType($e->nullable_taggable_id);
    \PHPStan\dumpType($e->uuid_nullable_taggable_type);
    \PHPStan\dumpType($e->uuid_nullable_taggable_id);
    \PHPStan\dumpType($e->uuid_taggable_type);
    \PHPStan\dumpType($e->uuid_taggable_id);
    \PHPStan\dumpType($e->deleted_at);
    \PHPStan\dumpType($e->year_col);
    \PHPStan\dumpType($e->remember_token);
    \PHPStan\dumpType($e->ip_address);
    \PHPStan\dumpType($e->user_id);
    \PHPStan\dumpType($e->link_ref);
    \PHPStan\dumpType($e->no_table_id);
    \PHPStan\dumpType($e->no_table_unsigned);
    \PHPStan\dumpType($e->created_at);
    \PHPStan\dumpType($e->updated_at);
    \PHPStan\dumpType($e->in_if);
    \PHPStan\dumpType($e->in_else);
    \PHPStan\dumpType($e->renamed);
    \PHPStan\dumpType($e->after_col);
    \PHPStan\dumpType($e->public_camel);
    \PHPStan\dumpType($e->not_accessor);
    \PHPStan\dumpType($e->legacy_attr);
    \PHPStan\dumpType($e->upper_accessor);
    \PHPStan\dumpType($c->uuid);
    \PHPStan\dumpType($c->score);
    \PHPStan\dumpType($c->created_at);
    \PHPStan\dumpType($n->id);
    \PHPStan\dumpType($t->name);
    \PHPStan\dumpType($t->after_rename);
    \PHPStan\dumpType($g->name);
    \PHPStan\dumpType($mu?->url);
    \PHPStan\dumpType($either->id);
    // attributes the plugin leaves alone: no column, dropped, dynamic, not a model
    $u->nope;
    $e->to_drop;
    $u->{$name};
    $std->id;
    $untyped->id;
}

/** Models without `$table`, whose table name Larastan derives from the class name. */
function probe_table_names(Models\Category $category, Models\Box $box, Models\Stats $stats): void
{
    \PHPStan\dumpType($category->id);
    \PHPStan\dumpType($category->position);
    \PHPStan\dumpType($box->label);
    \PHPStan\dumpType($stats->total);
}

/**
 * @template R
 * @param callable(): R $callback
 * @return R
 */
function callbackValue(callable $callback)
{
    return $callback();
}

function probe_callback_column(LinkUntagged $model): void
{
    \PHPStan\dumpType(callbackValue(static fn () => $model->url));
    \PHPStan\dumpType($model->url);
    $result = callbackValue(static function () use ($model) {
        \PHPStan\dumpType($model->url);
        return $model->url;
    });
    \PHPStan\dumpType($result);
}
