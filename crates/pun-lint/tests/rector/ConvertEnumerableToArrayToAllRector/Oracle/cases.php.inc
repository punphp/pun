<?php

namespace App;

use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Support\Collection;
use Illuminate\Support\Enumerable;

final class Plain {}
class Open {}
interface Iface {}
final class Arr implements Arrayable
{
    public function toArray(): array
    {
        return [];
    }
}
enum Suit
{
    case A;
}

/**
 * @template TKey of array-key
 * @template TValue
 * @extends Collection<TKey, TValue>
 */
class MyCollection extends Collection {}

/** @extends Collection<int, Plain> */
class PlainCollection extends Collection {}

/** @var Collection<int, string> $a */
$a->toArray();
/** @var Collection<int, Plain> $b */
$b->toArray();
/** @var Collection<int, Open> $c */
$c->toArray();
/** @var Collection<int, Iface> $d */
$d->toArray();
/** @var Collection<int, Arr> $e */
$e->toArray();
/** @var Collection<int, Suit> $f */
$f->toArray();
/** @var Collection<int, Plain|null> $g */
$g->toArray();
/** @var Collection<int, array<string>> $h */
$h->toArray();
/** @var Collection<int, \Closure> $i */
$i->toArray();
/** @var Collection<int, object> $j */
$j->toArray();
/** @var Collection $k */
$k->toArray();
/** @var Enumerable<int, int> $l */
$l->toArray();
/** @var MyCollection<int, int> $m */
$m->toArray();
/** @var PlainCollection $n */
$n->toArray();
/** @var Collection<int, Plain>|Collection<int, Open> $o */
$o->toArray();
/** @var Collection<int, Plain>|null $p */
$p?->toArray();
/** @var Collection<int, Unknown> $q */
$q->toArray();
/** @var Collection<int, int> $r */
$r->TOARRAY();
/** @var Collection<int, Plain|Suit> $s */
$s->toArray();
/** @var Collection<int, Arrayable> $t */
$t->toArray();
/** @var Collection<int, int>|Plain $u */
$u->toArray();
/** @var \ArrayObject $v */
$v->toArray();
/** @var Collection<int, int> $w */
$w->toArray()['a'];
/** @var Collection<int, Collection<int, int>> $x */
$x->toArray();
-----
<?php

namespace App;

use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Support\Collection;
use Illuminate\Support\Enumerable;

final class Plain {}
class Open {}
interface Iface {}
final class Arr implements Arrayable
{
    public function toArray(): array
    {
        return [];
    }
}
enum Suit
{
    case A;
}

/**
 * @template TKey of array-key
 * @template TValue
 * @extends Collection<TKey, TValue>
 */
class MyCollection extends Collection {}

/** @extends Collection<int, Plain> */
class PlainCollection extends Collection {}

/** @var Collection<int, string> $a */
$a->all();
/** @var Collection<int, Plain> $b */
$b->all();
/** @var Collection<int, Open> $c */
$c->toArray();
/** @var Collection<int, Iface> $d */
$d->toArray();
/** @var Collection<int, Arr> $e */
$e->toArray();
/** @var Collection<int, Suit> $f */
$f->all();
/** @var Collection<int, Plain|null> $g */
$g->all();
/** @var Collection<int, array<string>> $h */
$h->all();
/** @var Collection<int, \Closure> $i */
$i->all();
/** @var Collection<int, object> $j */
$j->toArray();
/** @var Collection $k */
$k->toArray();
/** @var Enumerable<int, int> $l */
$l->all();
/** @var MyCollection<int, int> $m */
$m->all();
/** @var PlainCollection $n */
$n->all();
/** @var Collection<int, Plain>|Collection<int, Open> $o */
$o->toArray();
/** @var Collection<int, Plain>|null $p */
$p?->all();
/** @var Collection<int, Unknown> $q */
$q->toArray();
/** @var Collection<int, int> $r */
$r->all();
/** @var Collection<int, Plain|Suit> $s */
$s->all();
/** @var Collection<int, Arrayable> $t */
$t->toArray();
/** @var Collection<int, int>|Plain $u */
$u->toArray();
/** @var \ArrayObject $v */
$v->toArray();
/** @var Collection<int, int> $w */
$w->all()['a'];
/** @var Collection<int, Collection<int, int>> $x */
$x->toArray();
