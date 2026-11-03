<?php

namespace Illuminate\Database\Query;

use Closure;
use DatePeriod;
use DateTimeInterface;
use Illuminate\Contracts\Database\Query\Builder as BuilderContract;
use Illuminate\Contracts\Database\Query\ConditionExpression;
use Illuminate\Contracts\Database\Query\Expression as ExpressionContract;
use Illuminate\Contracts\Support\Arrayable;
use Illuminate\Database\Concerns\BuildsQueries;
use Illuminate\Database\Concerns\BuildsWhereDateClauses;
use Illuminate\Database\Concerns\ExplainsQueries;
use Illuminate\Database\ConnectionInterface;
use Illuminate\Database\Eloquent\Builder as EloquentBuilder;
use Illuminate\Database\Eloquent\Relations\Relation;
use Illuminate\Database\Query\Grammars\Grammar;
use Illuminate\Database\Query\Processors\Processor;
use Illuminate\Pagination\Paginator;
use Illuminate\Support\Arr;
use Illuminate\Support\Collection;
use Illuminate\Support\LazyCollection;
use Illuminate\Support\Str;
use Illuminate\Support\Stringable;
use Illuminate\Support\Traits\ForwardsCalls;
use Illuminate\Support\Traits\Macroable;
use InvalidArgumentException;
use LogicException;
use RuntimeException;
use SortDirection;
use Traversable;
use UnitEnum;

use function Illuminate\Support\enum_value;

class Builder implements BuilderContract
{
    /** @use \Illuminate\Database\Concerns\BuildsQueries<\stdClass> */
    use BuildsWhereDateClauses, BuildsQueries, ExplainsQueries, ForwardsCalls, Macroable {
        __call as macroCall;
    }

    /**
     * The database connection instance.
     *
     * @var \Illuminate\Database\ConnectionInterface
     */
    public $connection;

    /**
     * The database query grammar instance.
     *
     * @var \Illuminate\Database\Query\Grammars\Grammar
     */
    public $grammar;

    /**
     * The database query post processor instance.
     *
     * @var \Illuminate\Database\Query\Processors\Processor
     */
    public $processor;

    /**
     * The current query value bindings.
     *
     * @var array{
     *     select: list<mixed>,
     *     from: list<mixed>,
     *     join: list<mixed>,
     *     where: list<mixed>,
     *     groupBy: list<mixed>,
     *     having: list<mixed>,
     *     order: list<mixed>,
     *     union: list<mixed>,
     *     unionOrder: list<mixed>,
     * }
     */
    public $bindings = [
        'select' => [],
        'from' => [],
        'join' => [],
        'where' => [],
        'groupBy' => [],
        'having' => [],
        'order' => [],
        'union' => [],
        'unionOrder' => [],
    ];

    /**
     * An aggregate function and column to be run.
     *
     * @var array{
     *     function: string,
     *     columns: array<\Illuminate\Contracts\Database\Query\Expression|string>
     * }|null
     */
    public $aggregate;

    /**
     * The columns that should be returned.
     *
     * @var array<string|\Illuminate\Contracts\Database\Query\Expression>|null
     */
    public $columns;

    /**
     * Indicates if the query returns distinct results.
     *
     * Occasionally contains the columns that should be distinct.
     *
     * @var bool|array
     */
    public $distinct = false;

    /**
     * The table which the query is targeting.
     *
     * @var \Illuminate\Database\Query\Expression|string
     */
    public $from;

    /**
     * The index hint for the query.
     *
     * @var \Illuminate\Database\Query\IndexHint|null
     */
    public $indexHint;

    /**
     * The table joins for the query.
     *
     * @var array|null
     */
    public $joins;

    /**
     * The where constraints for the query.
     *
     * @var array
     */
    public $wheres = [];

    /**
     * The groupings for the query.
     *
     * @var array|null
     */
    public $groups;

    /**
     * The having constraints for the query.
     *
     * @var array|null
     */
    public $havings;

    /**
     * The orderings for the query.
     *
     * @var array|null
     */
    public $orders;

    /**
     * The maximum number of records to return.
     *
     * @var int|null
     */
    public $limit;

    /**
     * The maximum number of records to return per group.
     *
     * @var array|null
     */
    public $groupLimit;

    /**
     * The number of records to skip.
     *
     * @var int|null
     */
    public $offset;

    /**
     * The query union statements.
     *
     * @var array|null
     */
    public $unions;

    /**
     * The maximum number of union records to return.
     *
     * @var int|null
     */
    public $unionLimit;

    /**
     * The number of union records to skip.
     *
     * @var int|null
     */
    public $unionOffset;

    /**
     * The orderings for the union query.
     *
     * @var array|null
     */
    public $unionOrders;

    /**
     * Indicates whether row locking is being used.
     *
     * @var string|bool|null
     */
    public $lock;

    /**
     * The query execution timeout in seconds.
     *
     * @var int|null
     */
    public $timeout;

    /**
     * The callbacks that should be invoked before the query is executed.
     *
     * @var array
     */
    public $beforeQueryCallbacks = [];

    /**
     * The callbacks that should be invoked after retrieving data from the database.
     *
     * @var array
     */
    protected $afterQueryCallbacks = [];

    /**
     * All of the available clause operators.
     *
     * @var string[]
     */
    public $operators = [
        '=', '<', '>', '<=', '>=', '<>', '!=', '<=>',
        'like', 'like binary', 'not like', 'ilike',
        '&', '|', '^', '<<', '>>', '&~', 'is', 'is not',
        'rlike', 'not rlike', 'regexp', 'not regexp',
        '~', '~*', '!~', '!~*', 'similar to',
        'not similar to', 'not ilike', '~~*', '!~~*',
    ];

    /**
     * All of the available bitwise operators.
     *
     * @var string[]
     */
    public $bitwiseOperators = [
        '&', '|', '^', '<<', '>>', '&~',
    ];

    /**
     * Whether to use write pdo for the select.
     *
     * @var bool
     */
    public $useWritePdo = false;

    /**
     * The custom arguments for the PDOStatement::fetchAll / fetch functions.
     *
     * @var array
     */
    public array $fetchUsing = [];

    /**
     * Create a new query builder instance.
     */
    public function __construct(
        ConnectionInterface $connection,
        ?Grammar $grammar = null,
        ?Processor $processor = null,
    ) {}

    /**
     * Set the columns to be selected.
     *
     * @param  mixed  $columns
     * @return $this
     */
    public function select($columns = ['*'])
    {}

    /**
     * Add a subselect expression to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|string  $query
     * @param  string  $as
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function selectSub($query, $as)
    {}

    /**
     * Add a select expression to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|literal-string  $expression
     * @param  string  $as
     * @return $this
     */
    public function selectExpression($expression, $as)
    {}

    /**
     * Add a new "raw" select expression to the query.
     *
     * @param  literal-string  $expression
     * @return $this
     */
    public function selectRaw($expression, array $bindings = [])
    {}

    /**
     * Makes "from" fetch from a subquery.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|string  $query
     * @param  string  $as
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function fromSub($query, $as)
    {}

    /**
     * Add a raw "from" clause to the query.
     *
     * @param  literal-string  $expression
     * @param  mixed  $bindings
     * @return $this
     */
    public function fromRaw($expression, $bindings = [])
    {}

    /**
     * Creates a subquery and parse it.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|string  $query
     * @return array
     */
    protected function createSub($query)
    {}

    /**
     * Parse the subquery into SQL and bindings.
     *
     * @param  mixed  $query
     * @return array
     *
     * @throws \InvalidArgumentException
     */
    protected function parseSub($query)
    {}

    /**
     * Prepend the database name if the given query is on another database.
     *
     * @param  mixed  $query
     * @return mixed
     */
    protected function prependDatabaseNameIfCrossDatabaseQuery($query)
    {}

    /**
     * Add a new select column to the query.
     *
     * @param  mixed  $column
     * @return $this
     */
    public function addSelect($column)
    {}

    /**
     * Add a vector-similarity selection to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \Illuminate\Support\Collection<int, float>|\Illuminate\Contracts\Support\Arrayable|array<int, float>|string  $vector
     * @param  string|null  $as
     * @return $this
     *
     * @throws \JsonException
     */
    public function selectVectorDistance($column, $vector, $as = null)
    {}

    /**
     * Force the query to only return distinct results.
     *
     * @return $this
     */
    public function distinct()
    {}

    /**
     * Set the table which the query is targeting.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|\Illuminate\Contracts\Database\Query\Expression|string  $table
     * @param  string|null  $as
     * @return $this
     */
    public function from($table, $as = null)
    {}

    /**
     * Add an index hint to suggest a query index.
     *
     * @param  string  $index
     * @return $this
     */
    public function useIndex($index)
    {}

    /**
     * Add an index hint to force a query index.
     *
     * @param  string  $index
     * @return $this
     */
    public function forceIndex($index)
    {}

    /**
     * Add an index hint to ignore a query index.
     *
     * @param  string  $index
     * @return $this
     */
    public function ignoreIndex($index)
    {}

    /**
     * Add a "join" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $table
     * @param  \Closure|\Illuminate\Contracts\Database\Query\Expression|string  $first
     * @param  string|null  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|null  $second
     * @param  string  $type
     * @param  bool  $where
     * @return $this
     */
    public function join($table, $first, $operator = null, $second = null, $type = 'inner', $where = false)
    {}

    /**
     * Add a "join where" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $table
     * @param  \Closure|\Illuminate\Contracts\Database\Query\Expression|string  $first
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $second
     * @param  string  $type
     * @return $this
     */
    public function joinWhere($table, $first, $operator, $second, $type = 'inner')
    {}

    /**
     * Add a "subquery join" clause to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|string  $query
     * @param  string  $as
     * @param  \Closure|\Illuminate\Contracts\Database\Query\Expression|string  $first
     * @param  string|null  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|null  $second
     * @param  string  $type
     * @param  bool  $where
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function joinSub($query, $as, $first, $operator = null, $second = null, $type = 'inner', $where = false)
    {}

    /**
     * Add a "lateral join" clause to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|string  $query
     * @return $this
     */
    public function joinLateral($query, string $as, string $type = 'inner')
    {}

    /**
     * Add a lateral left join to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|string  $query
     * @return $this
     */
    public function leftJoinLateral($query, string $as)
    {}

    /**
     * Add a left join to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $table
     * @param  \Closure|\Illuminate\Contracts\Database\Query\Expression|string  $first
     * @param  string|null  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|null  $second
     * @return $this
     */
    public function leftJoin($table, $first, $operator = null, $second = null)
    {}

    /**
     * Add a "join where" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $table
     * @param  \Closure|\Illuminate\Contracts\Database\Query\Expression|string  $first
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|null  $second
     * @return $this
     */
    public function leftJoinWhere($table, $first, $operator, $second)
    {}

    /**
     * Add a subquery left join to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|string  $query
     * @param  string  $as
     * @param  \Closure|\Illuminate\Contracts\Database\Query\Expression|string  $first
     * @param  string|null  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|null  $second
     * @return $this
     */
    public function leftJoinSub($query, $as, $first, $operator = null, $second = null)
    {}

    /**
     * Add a right join to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $table
     * @param  \Closure|string  $first
     * @param  string|null  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|null  $second
     * @return $this
     */
    public function rightJoin($table, $first, $operator = null, $second = null)
    {}

    /**
     * Add a "right join where" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $table
     * @param  \Closure|\Illuminate\Contracts\Database\Query\Expression|string  $first
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $second
     * @return $this
     */
    public function rightJoinWhere($table, $first, $operator, $second)
    {}

    /**
     * Add a subquery right join to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|string  $query
     * @param  string  $as
     * @param  \Closure|\Illuminate\Contracts\Database\Query\Expression|string  $first
     * @param  string|null  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|null  $second
     * @return $this
     */
    public function rightJoinSub($query, $as, $first, $operator = null, $second = null)
    {}

    /**
     * Add a "cross join" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $table
     * @param  \Closure|\Illuminate\Contracts\Database\Query\Expression|string|null  $first
     * @param  string|null  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|null  $second
     * @return $this
     */
    public function crossJoin($table, $first = null, $operator = null, $second = null)
    {}

    /**
     * Add a subquery cross join to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|string  $query
     * @param  string  $as
     * @return $this
     */
    public function crossJoinSub($query, $as)
    {}

    /**
     * Add a straight join to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $table
     * @param  \Closure|string  $first
     * @param  string|null  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|null  $second
     * @return $this
     */
    public function straightJoin($table, $first, $operator = null, $second = null)
    {}

    /**
     * Add a "straight join where" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $table
     * @param  \Closure|\Illuminate\Contracts\Database\Query\Expression|string  $first
     * @param  string  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $second
     * @return $this
     */
    public function straightJoinWhere($table, $first, $operator, $second)
    {}

    /**
     * Add a subquery straight join to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|string  $query
     * @param  string  $as
     * @param  \Closure|\Illuminate\Contracts\Database\Query\Expression|string  $first
     * @param  string|null  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|null  $second
     * @return $this
     */
    public function straightJoinSub($query, $as, $first, $operator = null, $second = null)
    {}

    /**
     * Get a new "join" clause.
     *
     * @param  string  $type
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $table
     * @return \Illuminate\Database\Query\JoinClause
     */
    protected function newJoinClause(self $parentQuery, $type, $table)
    {}

    /**
     * Get a new "join lateral" clause.
     *
     * @param  string  $type
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $table
     * @return \Illuminate\Database\Query\JoinLateralClause
     */
    protected function newJoinLateralClause(self $parentQuery, $type, $table)
    {}

    /**
     * Merge an array of "where" clauses and bindings.
     *
     * @param  array  $wheres
     * @param  array  $bindings
     * @return $this
     */
    public function mergeWheres($wheres, $bindings)
    {}

    /**
     * Add a basic "where" clause to the query.
     *
     * @param  \Closure|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    public function where($column, $operator = null, $value = null, $boolean = 'and')
    {}

    /**
     * Add an array of "where" clauses to the query.
     *
     * @param  array  $column
     * @param  string  $boolean
     * @param  string  $method
     * @return $this
     */
    protected function addArrayOfWheres($column, $boolean, $method = 'where')
    {}

    /**
     * Prepare the value and operator for a where clause.
     *
     * @param  string  $value
     * @param  string  $operator
     * @param  bool  $useDefault
     * @return array
     *
     * @throws \InvalidArgumentException
     */
    public function prepareValueAndOperator($value, $operator, $useDefault = false)
    {}

    /**
     * Determine if the given operator and value combination is legal.
     *
     * Prevents using Null values with invalid operators.
     *
     * @param  string  $operator
     * @param  mixed  $value
     * @return bool
     */
    protected function invalidOperatorAndValue($operator, $value)
    {}

    /**
     * Determine if the given operator is supported.
     *
     * @param  string  $operator
     * @return bool
     */
    protected function invalidOperator($operator)
    {}

    /**
     * Determine if the operator is a bitwise operator.
     *
     * @param  string  $operator
     * @return bool
     */
    protected function isBitwiseOperator($operator)
    {}

    /**
     * Add an "or where" clause to the query.
     *
     * @param  \Closure|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWhere($column, $operator = null, $value = null)
    {}

    /**
     * Add a basic "where not" clause to the query.
     *
     * @param  \Closure|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereNot($column, $operator = null, $value = null, $boolean = 'and')
    {}

    /**
     * Add an "or where not" clause to the query.
     *
     * @param  \Closure|string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereNot($column, $operator = null, $value = null)
    {}

    /**
     * Add a "where" clause comparing two columns to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|array  $first
     * @param  string|null  $operator
     * @param  string|null  $second
     * @param  string|null  $boolean
     * @return $this
     */
    public function whereColumn($first, $operator = null, $second = null, $boolean = 'and')
    {}

    /**
     * Add an "or where" clause comparing two columns to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|array  $first
     * @param  string|null  $operator
     * @param  string|null  $second
     * @return $this
     */
    public function orWhereColumn($first, $operator = null, $second = null)
    {}

    /**
     * Add a vector similarity clause to the query, filtering by minimum similarity and ordering by similarity.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \Illuminate\Support\Collection<int, float>|\Illuminate\Contracts\Support\Arrayable|array<int, float>|string  $vector
     * @param  float  $minSimilarity  A value between 0.0 and 1.0, where 1.0 is identical.
     * @param  bool  $order
     * @return $this
     */
    public function whereVectorSimilarTo($column, $vector, $minSimilarity = 0.6, $order = true)
    {}

    /**
     * Add a vector distance "where" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \Illuminate\Support\Collection<int, float>|\Illuminate\Contracts\Support\Arrayable|array<int, float>|string  $vector
     * @param  float  $maxDistance
     * @param  string  $boolean
     * @return $this
     *
     * @throws \JsonException
     */
    public function whereVectorDistanceLessThan($column, $vector, $maxDistance, $boolean = 'and')
    {}

    /**
     * Add a vector distance "or where" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \Illuminate\Support\Collection<int, float>|\Illuminate\Contracts\Support\Arrayable|array<int, float>|string  $vector
     * @param  float  $maxDistance
     * @return $this
     */
    public function orWhereVectorDistanceLessThan($column, $vector, $maxDistance)
    {}

    /**
     * Add a raw "where" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|literal-string  $sql
     * @param  mixed  $bindings
     * @param  string  $boolean
     * @return $this
     */
    public function whereRaw($sql, $bindings = [], $boolean = 'and')
    {}

    /**
     * Add a raw "or where" clause to the query.
     *
     * @param  literal-string  $sql
     * @param  mixed  $bindings
     * @return $this
     */
    public function orWhereRaw($sql, $bindings = [])
    {}

    /**
     * Add a "where binary" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $value
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function whereBinary($column, $value, $boolean = 'and', $not = false)
    {}

    /**
     * Add an "or where binary" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $value
     * @return $this
     */
    public function orWhereBinary($column, $value)
    {}

    /**
     * Add a "where not binary" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereNotBinary($column, $value, $boolean = 'and')
    {}

    /**
     * Add an "or where not binary" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $value
     * @return $this
     */
    public function orWhereNotBinary($column, $value)
    {}

    /**
     * Add a "where like" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $value
     * @param  bool  $caseSensitive
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function whereLike($column, $value, $caseSensitive = false, $boolean = 'and', $not = false)
    {}

    /**
     * Add an "or where like" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $value
     * @param  bool  $caseSensitive
     * @return $this
     */
    public function orWhereLike($column, $value, $caseSensitive = false)
    {}

    /**
     * Add a "where not like" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $value
     * @param  bool  $caseSensitive
     * @param  string  $boolean
     * @return $this
     */
    public function whereNotLike($column, $value, $caseSensitive = false, $boolean = 'and')
    {}

    /**
     * Add an "or where not like" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $value
     * @param  bool  $caseSensitive
     * @return $this
     */
    public function orWhereNotLike($column, $value, $caseSensitive = false)
    {}

    /**
     * Add a "where null safe equals" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereNullSafeEquals($column, $value, $boolean = 'and')
    {}

    /**
     * Add an "or where null safe equals" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereNullSafeEquals($column, $value)
    {}

    /**
     * Add a "where in" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  mixed  $values
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function whereIn($column, $values, $boolean = 'and', $not = false)
    {}

    /**
     * Add an "or where in" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  mixed  $values
     * @return $this
     */
    public function orWhereIn($column, $values)
    {}

    /**
     * Add a "where not in" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  mixed  $values
     * @param  string  $boolean
     * @return $this
     */
    public function whereNotIn($column, $values, $boolean = 'and')
    {}

    /**
     * Add an "or where not in" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  mixed  $values
     * @return $this
     */
    public function orWhereNotIn($column, $values)
    {}

    /**
     * Add a "where in raw" clause for integer values to the query.
     *
     * @param  string  $column
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $values
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function whereIntegerInRaw($column, $values, $boolean = 'and', $not = false)
    {}

    /**
     * Add an "or where in raw" clause for integer values to the query.
     *
     * @param  string  $column
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $values
     * @return $this
     */
    public function orWhereIntegerInRaw($column, $values)
    {}

    /**
     * Add a "where not in raw" clause for integer values to the query.
     *
     * @param  string  $column
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $values
     * @param  string  $boolean
     * @return $this
     */
    public function whereIntegerNotInRaw($column, $values, $boolean = 'and')
    {}

    /**
     * Add an "or where not in raw" clause for integer values to the query.
     *
     * @param  string  $column
     * @param  \Illuminate\Contracts\Support\Arrayable|array  $values
     * @return $this
     */
    public function orWhereIntegerNotInRaw($column, $values)
    {}

    /**
     * Add a "where null" clause to the query.
     *
     * @param  string|array|\Illuminate\Contracts\Database\Query\Expression  $columns
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function whereNull($columns, $boolean = 'and', $not = false)
    {}

    /**
     * Add an "or where null" clause to the query.
     *
     * @param  string|array|\Illuminate\Contracts\Database\Query\Expression  $column
     * @return $this
     */
    public function orWhereNull($column)
    {}

    /**
     * Add a "where not null" clause to the query.
     *
     * @param  string|array|\Illuminate\Contracts\Database\Query\Expression  $columns
     * @param  string  $boolean
     * @return $this
     */
    public function whereNotNull($columns, $boolean = 'and')
    {}

    /**
     * Add a "where between" statement to the query.
     *
     * @param  \Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|\Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function whereBetween($column, iterable $values, $boolean = 'and', $not = false)
    {}

    /**
     * Add a "where between" statement using columns to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function whereBetweenColumns($column, array $values, $boolean = 'and', $not = false)
    {}

    /**
     * Add an "or where between" statement to the query.
     *
     * @param  \Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|\Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return $this
     */
    public function orWhereBetween($column, iterable $values)
    {}

    /**
     * Add an "or where between" statement using columns to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return $this
     */
    public function orWhereBetweenColumns($column, array $values)
    {}

    /**
     * Add a "where not between" statement to the query.
     *
     * @param  \Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|\Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $boolean
     * @return $this
     */
    public function whereNotBetween($column, iterable $values, $boolean = 'and')
    {}

    /**
     * Add a "where not between" statement using columns to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $boolean
     * @return $this
     */
    public function whereNotBetweenColumns($column, array $values, $boolean = 'and')
    {}

    /**
     * Add an "or where not between" statement to the query.
     *
     * @param  \Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|\Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return $this
     */
    public function orWhereNotBetween($column, iterable $values)
    {}

    /**
     * Add an "or where not between" statement using columns to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return $this
     */
    public function orWhereNotBetweenColumns($column, array $values)
    {}

    /**
     * Add a "where between columns" statement using a value to the query.
     *
     * @param  mixed  $value
     * @param  array{\Illuminate\Contracts\Database\Query\Expression|string, \Illuminate\Contracts\Database\Query\Expression|string}  $columns
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function whereValueBetween($value, array $columns, $boolean = 'and', $not = false)
    {}

    /**
     * Add an "or where between columns" statement using a value to the query.
     *
     * @param  mixed  $value
     * @param  array{\Illuminate\Contracts\Database\Query\Expression|string, \Illuminate\Contracts\Database\Query\Expression|string}  $columns
     * @return $this
     */
    public function orWhereValueBetween($value, array $columns)
    {}

    /**
     * Add a "where not between columns" statement using a value to the query.
     *
     * @param  mixed  $value
     * @param  array{\Illuminate\Contracts\Database\Query\Expression|string, \Illuminate\Contracts\Database\Query\Expression|string}  $columns
     * @param  string  $boolean
     * @return $this
     */
    public function whereValueNotBetween($value, array $columns, $boolean = 'and')
    {}

    /**
     * Add an "or where not between columns" statement using a value to the query.
     *
     * @param  mixed  $value
     * @param  array{\Illuminate\Contracts\Database\Query\Expression|string, \Illuminate\Contracts\Database\Query\Expression|string}  $columns
     * @return $this
     */
    public function orWhereValueNotBetween($value, array $columns)
    {}

    /**
     * Add an "or where not null" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return $this
     */
    public function orWhereNotNull($column)
    {}

    /**
     * Add a "where date" statement to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \DateTimeInterface|string|null  $operator
     * @param  \DateTimeInterface|string|null  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereDate($column, $operator, $value = null, $boolean = 'and')
    {}

    /**
     * Add an "or where date" statement to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \DateTimeInterface|string|null  $operator
     * @param  \DateTimeInterface|string|null  $value
     * @return $this
     */
    public function orWhereDate($column, $operator, $value = null)
    {}

    /**
     * Add a "where time" statement to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \DateTimeInterface|string|null  $operator
     * @param  \DateTimeInterface|string|null  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereTime($column, $operator, $value = null, $boolean = 'and')
    {}

    /**
     * Add an "or where time" statement to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \DateTimeInterface|string|null  $operator
     * @param  \DateTimeInterface|string|null  $value
     * @return $this
     */
    public function orWhereTime($column, $operator, $value = null)
    {}

    /**
     * Add a "where day" statement to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \DateTimeInterface|string|int|null  $operator
     * @param  \DateTimeInterface|string|int|null  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereDay($column, $operator, $value = null, $boolean = 'and')
    {}

    /**
     * Add an "or where day" statement to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \DateTimeInterface|string|int|null  $operator
     * @param  \DateTimeInterface|string|int|null  $value
     * @return $this
     */
    public function orWhereDay($column, $operator, $value = null)
    {}

    /**
     * Add a "where month" statement to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \DateTimeInterface|string|int|null  $operator
     * @param  \DateTimeInterface|string|int|null  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereMonth($column, $operator, $value = null, $boolean = 'and')
    {}

    /**
     * Add an "or where month" statement to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \DateTimeInterface|string|int|null  $operator
     * @param  \DateTimeInterface|string|int|null  $value
     * @return $this
     */
    public function orWhereMonth($column, $operator, $value = null)
    {}

    /**
     * Add a "where year" statement to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \DateTimeInterface|string|int|null  $operator
     * @param  \DateTimeInterface|string|int|null  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereYear($column, $operator, $value = null, $boolean = 'and')
    {}

    /**
     * Add an "or where year" statement to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \DateTimeInterface|string|int|null  $operator
     * @param  \DateTimeInterface|string|int|null  $value
     * @return $this
     */
    public function orWhereYear($column, $operator, $value = null)
    {}

    /**
     * Add a date based (year, month, day, time) statement to the query.
     *
     * @param  string  $type
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    protected function addDateBasedWhere($type, $column, $operator, $value, $boolean = 'and')
    {}

    /**
     * Add a nested "where" statement to the query.
     *
     * @param  string  $boolean
     * @return $this
     */
    public function whereNested(Closure $callback, $boolean = 'and')
    {}

    /**
     * Create a new query instance for nested where condition.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    public function forNestedWhere()
    {}

    /**
     * Add another query builder as a nested where to the query builder.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @param  string  $boolean
     * @return $this
     */
    public function addNestedWhereQuery($query, $boolean = 'and')
    {}

    /**
     * Add a full sub-select to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string  $operator
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>  $callback
     * @param  string  $boolean
     * @return $this
     */
    protected function whereSub($column, $operator, $callback, $boolean)
    {}

    /**
     * Add an "exists" clause to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>  $callback
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function whereExists($callback, $boolean = 'and', $not = false)
    {}

    /**
     * Add an "or where exists" clause to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>  $callback
     * @param  bool  $not
     * @return $this
     */
    public function orWhereExists($callback, $not = false)
    {}

    /**
     * Add a "where not exists" clause to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>  $callback
     * @param  string  $boolean
     * @return $this
     */
    public function whereNotExists($callback, $boolean = 'and')
    {}

    /**
     * Add an "or where not exists" clause to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>  $callback
     * @return $this
     */
    public function orWhereNotExists($callback)
    {}

    /**
     * Add an "exists" clause to the query.
     *
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function addWhereExistsQuery(self $query, $boolean = 'and', $not = false)
    {}

    /**
     * Adds a where condition using row values.
     *
     * @param  array  $columns
     * @param  string  $operator
     * @param  array  $values
     * @param  string  $boolean
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function whereRowValues($columns, $operator, $values, $boolean = 'and')
    {}

    /**
     * Adds an or where condition using row values.
     *
     * @param  array  $columns
     * @param  string  $operator
     * @param  array  $values
     * @return $this
     */
    public function orWhereRowValues($columns, $operator, $values)
    {}

    /**
     * Add a "where JSON contains" clause to the query.
     *
     * @param  string  $column
     * @param  mixed  $value
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function whereJsonContains($column, $value, $boolean = 'and', $not = false)
    {}

    /**
     * Add an "or where JSON contains" clause to the query.
     *
     * @param  string  $column
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereJsonContains($column, $value)
    {}

    /**
     * Add a "where JSON not contains" clause to the query.
     *
     * @param  string  $column
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereJsonDoesntContain($column, $value, $boolean = 'and')
    {}

    /**
     * Add an "or where JSON not contains" clause to the query.
     *
     * @param  string  $column
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereJsonDoesntContain($column, $value)
    {}

    /**
     * Add a "where JSON overlaps" clause to the query.
     *
     * @param  string  $column
     * @param  mixed  $value
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function whereJsonOverlaps($column, $value, $boolean = 'and', $not = false)
    {}

    /**
     * Add an "or where JSON overlaps" clause to the query.
     *
     * @param  string  $column
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereJsonOverlaps($column, $value)
    {}

    /**
     * Add a "where JSON not overlap" clause to the query.
     *
     * @param  string  $column
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereJsonDoesntOverlap($column, $value, $boolean = 'and')
    {}

    /**
     * Add an "or where JSON not overlap" clause to the query.
     *
     * @param  string  $column
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereJsonDoesntOverlap($column, $value)
    {}

    /**
     * Add a clause that determines if a JSON path exists to the query.
     *
     * @param  string  $column
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function whereJsonContainsKey($column, $boolean = 'and', $not = false)
    {}

    /**
     * Add an "or" clause that determines if a JSON path exists to the query.
     *
     * @param  string  $column
     * @return $this
     */
    public function orWhereJsonContainsKey($column)
    {}

    /**
     * Add a clause that determines if a JSON path does not exist to the query.
     *
     * @param  string  $column
     * @param  string  $boolean
     * @return $this
     */
    public function whereJsonDoesntContainKey($column, $boolean = 'and')
    {}

    /**
     * Add an "or" clause that determines if a JSON path does not exist to the query.
     *
     * @param  string  $column
     * @return $this
     */
    public function orWhereJsonDoesntContainKey($column)
    {}

    /**
     * Add a "where JSON length" clause to the query.
     *
     * @param  string  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereJsonLength($column, $operator, $value = null, $boolean = 'and')
    {}

    /**
     * Add an "or where JSON length" clause to the query.
     *
     * @param  string  $column
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereJsonLength($column, $operator, $value = null)
    {}

    /**
     * Handles dynamic "where" clauses to the query.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return $this
     */
    public function dynamicWhere($method, $parameters)
    {}

    /**
     * Add a single dynamic "where" clause statement to the query.
     *
     * @param  string  $segment
     * @param  string  $connector
     * @param  array  $parameters
     * @param  int  $index
     * @return void
     */
    protected function addDynamic($segment, $connector, $parameters, $index)
    {}

    /**
     * Add a "where fulltext" clause to the query.
     *
     * @param  string|string[]  $columns
     * @param  string  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereFullText($columns, $value, array $options = [], $boolean = 'and')
    {}

    /**
     * Add an "or where fulltext" clause to the query.
     *
     * @param  string|string[]  $columns
     * @param  string  $value
     * @return $this
     */
    public function orWhereFullText($columns, $value, array $options = [])
    {}

    /**
     * Add a "where" clause to the query for multiple columns with "and" conditions between them.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression[]|\Closure[]|string[]  $columns
     * @param  mixed  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereAll($columns, $operator = null, $value = null, $boolean = 'and')
    {}

    /**
     * Add an "or where" clause to the query for multiple columns with "and" conditions between them.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression[]|\Closure[]|string[]  $columns
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereAll($columns, $operator = null, $value = null)
    {}

    /**
     * Add a "where" clause to the query for multiple columns with "or" conditions between them.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression[]|\Closure[]|string[]  $columns
     * @param  mixed  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereAny($columns, $operator = null, $value = null, $boolean = 'and')
    {}

    /**
     * Add an "or where" clause to the query for multiple columns with "or" conditions between them.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression[]|\Closure[]|string[]  $columns
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereAny($columns, $operator = null, $value = null)
    {}

    /**
     * Add a "where not" clause to the query for multiple columns where none of the conditions should be true.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression[]|\Closure[]|string[]  $columns
     * @param  mixed  $operator
     * @param  mixed  $value
     * @param  string  $boolean
     * @return $this
     */
    public function whereNone($columns, $operator = null, $value = null, $boolean = 'and')
    {}

    /**
     * Add an "or where not" clause to the query for multiple columns where none of the conditions should be true.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression[]|\Closure[]|string[]  $columns
     * @param  mixed  $operator
     * @param  mixed  $value
     * @return $this
     */
    public function orWhereNone($columns, $operator = null, $value = null)
    {}

    /**
     * Add a "group by" clause to the query.
     *
     * @param  array|\Illuminate\Contracts\Database\Query\Expression|string  ...$groups
     * @return $this
     */
    public function groupBy(...$groups)
    {}

    /**
     * Add a raw "groupBy" clause to the query.
     *
     * @param  literal-string  $sql
     * @return $this
     */
    public function groupByRaw($sql, array $bindings = [])
    {}

    /**
     * Add a "having" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|\Closure|string  $column
     * @param  \DateTimeInterface|string|int|float|null  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|\DateTimeInterface|string|int|float|null  $value
     * @param  string  $boolean
     * @return $this
     */
    public function having($column, $operator = null, $value = null, $boolean = 'and')
    {}

    /**
     * Add an "or having" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|\Closure|string  $column
     * @param  \DateTimeInterface|string|int|float|null  $operator
     * @param  \Illuminate\Contracts\Database\Query\Expression|\DateTimeInterface|string|int|float|null  $value
     * @return $this
     */
    public function orHaving($column, $operator = null, $value = null)
    {}

    /**
     * Add a nested "having" statement to the query.
     *
     * @param  string  $boolean
     * @return $this
     */
    public function havingNested(Closure $callback, $boolean = 'and')
    {}

    /**
     * Add another query builder as a nested having to the query builder.
     *
     * @param  \Illuminate\Database\Query\Builder  $query
     * @param  string  $boolean
     * @return $this
     */
    public function addNestedHavingQuery($query, $boolean = 'and')
    {}

    /**
     * Add a "having null" clause to the query.
     *
     * @param  array|string  $columns
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function havingNull($columns, $boolean = 'and', $not = false)
    {}

    /**
     * Add an "or having null" clause to the query.
     *
     * @param  string  $column
     * @return $this
     */
    public function orHavingNull($column)
    {}

    /**
     * Add a "having not null" clause to the query.
     *
     * @param  array|string  $columns
     * @param  string  $boolean
     * @return $this
     */
    public function havingNotNull($columns, $boolean = 'and')
    {}

    /**
     * Add an "or having not null" clause to the query.
     *
     * @param  string  $column
     * @return $this
     */
    public function orHavingNotNull($column)
    {}

    /**
     * Add a "having between" clause to the query.
     *
     * @param  string  $column
     * @param  string  $boolean
     * @param  bool  $not
     * @return $this
     */
    public function havingBetween($column, iterable $values, $boolean = 'and', $not = false)
    {}

    /**
     * Add a "having not between" clause to the query.
     *
     * @param  string  $column
     * @param  iterable  $values
     * @param  string  $boolean
     * @return $this
     */
    public function havingNotBetween($column, iterable $values, $boolean = 'and')
    {}

    /**
     * Add an "or having between" clause to the query.
     *
     * @param  string  $column
     * @param  iterable  $values
     * @return $this
     */
    public function orHavingBetween($column, iterable $values)
    {}

    /**
     * Add an "or having not between" clause to the query.
     *
     * @param  string  $column
     * @param  iterable  $values
     * @return $this
     */
    public function orHavingNotBetween($column, iterable $values)
    {}

    /**
     * Resolve the start and end dates from a DatePeriod.
     *
     * @param  \DatePeriod  $period
     * @return array{\DateTimeInterface, \DateTimeInterface}
     */
    protected function resolveDatePeriodBounds(DatePeriod $period)
    {}

    /**
     * Add a raw "having" clause to the query.
     *
     * @param  literal-string  $sql
     * @param  string  $boolean
     * @return $this
     */
    public function havingRaw($sql, array $bindings = [], $boolean = 'and')
    {}

    /**
     * Add a raw "or having" clause to the query.
     *
     * @param  literal-string  $sql
     * @return $this
     */
    public function orHavingRaw($sql, array $bindings = [])
    {}

    /**
     * Add an "order by" clause to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|\Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  SortDirection|'asc'|'desc'  $direction
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function orderBy($column, $direction = SortDirection::Ascending)
    {}

    /**
     * Add a descending "order by" clause to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|\Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return $this
     */
    public function orderByDesc($column)
    {}

    /**
     * Add an "order by" clause for a timestamp to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return $this
     */
    public function latest($column = 'created_at')
    {}

    /**
     * Add an "order by" clause for a timestamp to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return $this
     */
    public function oldest($column = 'created_at')
    {}

    /**
     * Add a vector-distance "order by" clause to the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \Illuminate\Support\Collection<int, float>|\Illuminate\Contracts\Support\Arrayable|array<int, float>|string  $vector
     * @return $this
     *
     * @throws \JsonException
     */
    public function orderByVectorDistance($column, $vector)
    {}

    /**
     * Put the query's results in random order.
     *
     * @param  string|int  $seed
     * @return $this
     */
    public function inRandomOrder($seed = '')
    {}

    /**
     * Add an "order by" clause to order results by a given sequence of values.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  \Illuminate\Contracts\Support\Arrayable|array<\UnitEnum|string|int|float|bool>  $values
     * @return $this
     */
    public function inOrderOf($column, $values)
    {}

    /**
     * Add a raw "order by" clause to the query.
     *
     * @param  literal-string  $sql
     * @param  array  $bindings
     * @return $this
     */
    public function orderByRaw($sql, $bindings = [])
    {}

    /**
     * Alias to set the "offset" value of the query.
     *
     * @param  int  $value
     * @return $this
     */
    public function skip($value)
    {}

    /**
     * Set the "offset" value of the query.
     *
     * @param  int  $value
     * @return $this
     */
    public function offset($value)
    {}

    /**
     * Alias to set the "limit" value of the query.
     *
     * @param  int|null  $value
     * @return $this
     */
    public function take($value)
    {}

    /**
     * Set the "limit" value of the query.
     *
     * @param  int|null  $value
     * @return $this
     */
    public function limit($value)
    {}

    /**
     * Add a "group limit" clause to the query.
     *
     * @param  int  $value
     * @param  string  $column
     * @return $this
     */
    public function groupLimit($value, $column)
    {}

    /**
     * Set the limit and offset for a given page.
     *
     * @param  int  $page
     * @param  int  $perPage
     * @return $this
     */
    public function forPage($page, $perPage = 15)
    {}

    /**
     * Constrain the query to the previous "page" of results before a given ID.
     *
     * @param  int  $perPage
     * @param  int|null  $lastId
     * @param  string  $column
     * @return $this
     */
    public function forPageBeforeId($perPage = 15, $lastId = 0, $column = 'id')
    {}

    /**
     * Constrain the query to the next "page" of results after a given ID.
     *
     * @param  int  $perPage
     * @param  int|null  $lastId
     * @param  string  $column
     * @return $this
     */
    public function forPageAfterId($perPage = 15, $lastId = 0, $column = 'id')
    {}

    /**
     * Remove all existing orders and optionally add a new order.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Contracts\Database\Query\Expression|string|null  $column
     * @param  SortDirection|'asc'|'desc'  $direction
     * @return $this
     */
    public function reorder($column = null, $direction = SortDirection::Ascending)
    {}

    /**
     * Add descending "reorder" clause to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Contracts\Database\Query\Expression|string|null  $column
     * @return $this
     */
    public function reorderDesc($column)
    {}

    /**
     * Get an array with all orders with a given column removed.
     *
     * @param  string  $column
     * @return array
     */
    protected function removeExistingOrdersFor($column)
    {}

    /**
     * Add a "union" statement to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>  $query
     * @param  bool  $all
     * @return $this
     */
    public function union($query, $all = false)
    {}

    /**
     * Add a "union all" statement to the query.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>  $query
     * @return $this
     */
    public function unionAll($query)
    {}

    /**
     * Lock the selected rows in the table.
     *
     * @param  string|bool  $value
     * @return $this
     */
    public function lock($value = true)
    {}

    /**
     * Lock the selected rows in the table for updating.
     *
     * @return $this
     */
    public function lockForUpdate()
    {}

    /**
     * Share lock the selected rows in the table.
     *
     * @return $this
     */
    public function sharedLock()
    {}

    /**
     * Set a query execution timeout in seconds.
     *
     * @param  int|null  $seconds
     * @return $this
     *
     * @throws InvalidArgumentException
     */
    public function timeout(?int $seconds): static
    {}

    /**
     * Register a closure to be invoked before the query is executed.
     *
     * @return $this
     */
    public function beforeQuery(callable $callback)
    {}

    /**
     * Invoke the "before query" modification callbacks.
     *
     * @return void
     */
    public function applyBeforeQueryCallbacks()
    {}

    /**
     * Register a closure to be invoked after the query is executed.
     *
     * @return $this
     */
    public function afterQuery(Closure $callback)
    {}

    /**
     * Invoke the "after query" modification callbacks.
     *
     * @param  mixed  $result
     * @return mixed
     */
    public function applyAfterQueryCallbacks($result)
    {}

    /**
     * Get the SQL representation of the query.
     *
     * @return string
     */
    public function toSql()
    {}

    /**
     * Get the raw SQL representation of the query with embedded bindings.
     *
     * @return string
     */
    public function toRawSql()
    {}

    /**
     * Execute a query for a single record by ID.
     *
     * @param  int|string  $id
     * @param  string|\Illuminate\Contracts\Database\Query\Expression|array<string|\Illuminate\Contracts\Database\Query\Expression>  $columns
     * @return \stdClass|null
     */
    public function find($id, $columns = ['*'])
    {}

    /**
     * Execute a query for a single record by ID or call a callback.
     *
     * @template TValue
     *
     * @param  mixed  $id
     * @param  (\Closure(): TValue)|string|\Illuminate\Contracts\Database\Query\Expression|array<string|\Illuminate\Contracts\Database\Query\Expression>  $columns
     * @param  (\Closure(): TValue)|null  $callback
     * @return \stdClass|TValue
     */
    public function findOr($id, $columns = ['*'], ?Closure $callback = null)
    {}

    /**
     * Get a single column's value from the first result of a query.
     *
     * @param  string  $column
     * @return mixed
     */
    public function value($column)
    {}

    /**
     * Get a single expression value from the first result of a query.
     *
     * @param  literal-string  $expression
     * @return mixed
     */
    public function rawValue(string $expression, array $bindings = [])
    {}

    /**
     * Get a single column's value from the first result of a query if it's the sole matching record.
     *
     * @param  string  $column
     * @return mixed
     *
     * @throws \Illuminate\Database\RecordsNotFoundException
     * @throws \Illuminate\Database\MultipleRecordsFoundException
     */
    public function soleValue($column)
    {}

    /**
     * Execute the query as a "select" statement.
     *
     * @param  string|\Illuminate\Contracts\Database\Query\Expression|array<string|\Illuminate\Contracts\Database\Query\Expression>  $columns
     * @return \Illuminate\Support\Collection<int, \stdClass>
     */
    public function get($columns = ['*'])
    {}

    /**
     * Run the query as a "select" statement against the connection.
     *
     * @return array
     */
    protected function runSelect()
    {}

    /**
     * Remove the group limit keys from the results in the collection.
     *
     * @param  \Illuminate\Support\Collection  $items
     * @return \Illuminate\Support\Collection
     */
    protected function withoutGroupLimitKeys($items)
    {}

    /**
     * Paginate the given query into a simple paginator.
     *
     * @param  int|\Closure  $perPage
     * @param  string|\Illuminate\Contracts\Database\Query\Expression|array<string|\Illuminate\Contracts\Database\Query\Expression>  $columns
     * @param  string  $pageName
     * @param  int|null  $page
     * @param  \Closure|int|null  $total
     * @return \Illuminate\Pagination\LengthAwarePaginator
     */
    public function paginate($perPage = 15, $columns = ['*'], $pageName = 'page', $page = null, $total = null)
    {}

    /**
     * Get a paginator only supporting simple next and previous links.
     *
     * This is more efficient on larger data-sets, etc.
     *
     * @param  int  $perPage
     * @param  string|\Illuminate\Contracts\Database\Query\Expression|array<string|\Illuminate\Contracts\Database\Query\Expression>  $columns
     * @param  string  $pageName
     * @param  int|null  $page
     * @return \Illuminate\Contracts\Pagination\Paginator
     */
    public function simplePaginate($perPage = 15, $columns = ['*'], $pageName = 'page', $page = null)
    {}

    /**
     * Get a paginator only supporting simple next and previous links.
     *
     * This is more efficient on larger data-sets, etc.
     *
     * @param  int|null  $perPage
     * @param  string|\Illuminate\Contracts\Database\Query\Expression|array<string|\Illuminate\Contracts\Database\Query\Expression>  $columns
     * @param  string  $cursorName
     * @param  \Illuminate\Pagination\Cursor|string|null  $cursor
     * @return \Illuminate\Contracts\Pagination\CursorPaginator
     */
    public function cursorPaginate($perPage = 15, $columns = ['*'], $cursorName = 'cursor', $cursor = null)
    {}

    /**
     * Ensure the proper order by required for cursor pagination.
     *
     * @param  bool  $shouldReverse
     * @return \Illuminate\Support\Collection
     */
    protected function ensureOrderForCursorPagination($shouldReverse = false)
    {}

    /**
     * Get the count of the total records for the paginator.
     *
     * @param  array<string|\Illuminate\Contracts\Database\Query\Expression>  $columns
     * @return int<0, max>
     */
    public function getCountForPagination($columns = ['*'])
    {}

    /**
     * Run a pagination count query.
     *
     * @param  array<string|\Illuminate\Contracts\Database\Query\Expression>  $columns
     * @return array<mixed>
     */
    protected function runPaginationCountQuery($columns = ['*'])
    {}

    /**
     * Clone the existing query instance for usage in a pagination subquery.
     *
     * @return self
     */
    protected function cloneForPaginationCount()
    {}

    /**
     * Remove the column aliases since they will break count queries.
     *
     * @param  array<string|\Illuminate\Contracts\Database\Query\Expression>  $columns
     * @return array<string|\Illuminate\Contracts\Database\Query\Expression>
     */
    protected function withoutSelectAliases(array $columns)
    {}

    /**
     * Get a lazy collection for the given query.
     *
     * @return \Illuminate\Support\LazyCollection<int, \stdClass>
     */
    public function cursor()
    {}

    /**
     * Throw an exception if the query doesn't have an orderBy clause.
     *
     * @return void
     *
     * @throws \RuntimeException
     */
    protected function enforceOrderBy()
    {}

    /**
     * Get a collection instance containing the values of a given column.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @param  string|null  $key
     * @return \Illuminate\Support\Collection<array-key, mixed>
     */
    public function pluck($column, $key = null)
    {}

    /**
     * Strip off the table name or alias from a column identifier.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string|null  $column
     * @return string|null
     */
    protected function stripTableForPluck($column)
    {}

    /**
     * Retrieve column values from rows represented as objects.
     *
     * @param  array  $queryResult
     * @param  string  $column
     * @param  string|null  $key
     * @return \Illuminate\Support\Collection
     */
    protected function pluckFromObjectColumn($queryResult, $column, $key)
    {}

    /**
     * Retrieve column values from rows represented as arrays.
     *
     * @param  array  $queryResult
     * @param  string  $column
     * @param  string|null  $key
     * @return \Illuminate\Support\Collection
     */
    protected function pluckFromArrayColumn($queryResult, $column, $key)
    {}

    /**
     * Concatenate values of a given column as a string.
     *
     * @param  string  $column
     * @param  string  $glue
     * @return string
     */
    public function implode($column, $glue = '')
    {}

    /**
     * Determine if any rows exist for the current query.
     *
     * @return bool
     */
    public function exists()
    {}

    /**
     * Determine if no rows exist for the current query.
     *
     * @return bool
     */
    public function doesntExist()
    {}

    /**
     * Execute the given callback if no rows exist for the current query.
     *
     * @return mixed
     */
    public function existsOr(Closure $callback)
    {}

    /**
     * Execute the given callback if rows exist for the current query.
     *
     * @return mixed
     */
    public function doesntExistOr(Closure $callback)
    {}

    /**
     * Retrieve the "count" result of the query.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $columns
     * @return int<0, max>
     */
    public function count($columns = '*')
    {}

    /**
     * Retrieve the minimum value of a given column.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return mixed
     */
    public function min($column)
    {}

    /**
     * Retrieve the maximum value of a given column.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return mixed
     */
    public function max($column)
    {}

    /**
     * Retrieve the sum of the values of a given column.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return mixed
     */
    public function sum($column)
    {}

    /**
     * Retrieve the average of the values of a given column.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return mixed
     */
    public function avg($column)
    {}

    /**
     * Alias for the "avg" method.
     *
     * @param  \Illuminate\Contracts\Database\Query\Expression|string  $column
     * @return mixed
     */
    public function average($column)
    {}

    /**
     * Execute an aggregate function on the database.
     *
     * @param  string  $function
     * @param  array  $columns
     * @return mixed
     */
    public function aggregate($function, $columns = ['*'])
    {}

    /**
     * Execute a numeric aggregate function on the database.
     *
     * @param  string  $function
     * @param  array  $columns
     * @return float|int
     */
    public function numericAggregate($function, $columns = ['*'])
    {}

    /**
     * Set the aggregate property without running the query.
     *
     * @param  string  $function
     * @param  array<\Illuminate\Contracts\Database\Query\Expression|string>  $columns
     * @return $this
     */
    protected function setAggregate($function, $columns)
    {}

    /**
     * Insert new records into the database.
     *
     * @return bool
     */
    public function insert(array $values)
    {}

    /**
     * Insert new records into the database while ignoring errors.
     *
     * @return int<0, max>
     */
    public function insertOrIgnore(array $values)
    {}

    /**
     * Insert new records into the database and returning specified columns with optional ignoring specific conflicts.
     *
     * @param  non-empty-array<non-empty-string>  $returning
     * @param  non-empty-string|non-empty-array<non-empty-string>|null  $uniqueBy
     * @return \Illuminate\Support\Collection
     */
    public function insertOrIgnoreReturning(array $values, array $returning = ['*'], array|string|null $uniqueBy = null)
    {}

    /**
     * Insert a new record and get the value of the primary key.
     *
     * @param  string|null  $sequence
     * @return int
     */
    public function insertGetId(array $values, $sequence = null)
    {}

    /**
     * Insert new records into the table using a subquery.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|string  $query
     * @return int
     */
    public function insertUsing(array $columns, $query)
    {}

    /**
     * Insert new records into the table using a subquery while ignoring errors.
     *
     * @param  \Closure|\Illuminate\Database\Query\Builder|\Illuminate\Database\Eloquent\Builder<*>|string  $query
     * @return int
     */
    public function insertOrIgnoreUsing(array $columns, $query)
    {}

    /**
     * Update records in the database.
     *
     * @return int<0, max>
     */
    public function update(array $values)
    {}

    /**
     * Update records in a PostgreSQL database using the update from syntax.
     *
     * @return int
     *
     * @throws \LogicException
     */
    public function updateFrom(array $values)
    {}

    /**
     * Insert or update a record matching the attributes, and fill it with values.
     *
     * @return bool
     */
    public function updateOrInsert(array $attributes, array|callable $values = [])
    {}

    /**
     * Insert new records or update the existing ones.
     *
     * @param  non-empty-string|non-empty-array<int, non-empty-string>  $uniqueBy
     * @return int
     */
    public function upsert(array $values, array|string $uniqueBy, ?array $update = null)
    {}

    /**
     * Increment a column's value by a given amount.
     *
     * @param  string  $column
     * @param  float|int  $amount
     * @return int<0, max>
     *
     * @throws \InvalidArgumentException
     */
    public function increment($column, $amount = 1, array $extra = [])
    {}

    /**
     * Increment the given column's values by the given amounts.
     *
     * @param  array<string, float|int|numeric-string>  $columns
     * @param  array<string, mixed>  $extra
     * @return int<0, max>
     *
     * @throws \InvalidArgumentException
     */
    public function incrementEach(array $columns, array $extra = [])
    {}

    /**
     * Decrement a column's value by a given amount.
     *
     * @param  string  $column
     * @param  float|int  $amount
     * @return int<0, max>
     *
     * @throws \InvalidArgumentException
     */
    public function decrement($column, $amount = 1, array $extra = [])
    {}

    /**
     * Decrement the given column's values by the given amounts.
     *
     * @param  array<string, float|int|numeric-string>  $columns
     * @param  array<string, mixed>  $extra
     * @return int<0, max>
     *
     * @throws \InvalidArgumentException
     */
    public function decrementEach(array $columns, array $extra = [])
    {}

    /**
     * Delete records from the database.
     *
     * @param  mixed  $id
     * @return int
     */
    public function delete($id = null)
    {}

    /**
     * Run a "truncate" statement on the table.
     *
     * @return void
     */
    public function truncate()
    {}

    /**
     * Get a new instance of the query builder.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    public function newQuery()
    {}

    /**
     * Create a new query instance for a sub-query.
     *
     * @return \Illuminate\Database\Query\Builder
     */
    protected function forSubQuery()
    {}

    /**
     * Get all of the query builder's columns in a text-only array with all expressions evaluated.
     *
     * @return list<string>
     */
    public function getColumns()
    {}

    /**
     * Create a raw database expression.
     *
     * @param  literal-string|int|float  $value
     * @return \Illuminate\Contracts\Database\Query\Expression
     */
    public function raw($value)
    {}

    /**
     * Get the query builder instances that are used in the union of the query.
     *
     * @return \Illuminate\Support\Collection
     */
    protected function getUnionBuilders()
    {}

    /**
     * Get the "limit" value for the query or null if it's not set.
     *
     * @return mixed
     */
    public function getLimit()
    {}

    /**
     * Get the "offset" value for the query or null if it's not set.
     *
     * @return mixed
     */
    public function getOffset()
    {}

    /**
     * Get the current query value bindings in a flattened array.
     *
     * @return list<mixed>
     */
    public function getBindings()
    {}

    /**
     * Get the raw array of bindings.
     *
     * @return array{
     *      select: list<mixed>,
     *      from: list<mixed>,
     *      join: list<mixed>,
     *      where: list<mixed>,
     *      groupBy: list<mixed>,
     *      having: list<mixed>,
     *      order: list<mixed>,
     *      union: list<mixed>,
     *      unionOrder: list<mixed>,
     * }
     */
    public function getRawBindings()
    {}

    /**
     * Set the bindings on the query builder.
     *
     * @param  list<mixed>  $bindings
     * @param  "select"|"from"|"join"|"where"|"groupBy"|"having"|"order"|"union"|"unionOrder"  $type
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function setBindings(array $bindings, $type = 'where')
    {}

    /**
     * Add a binding to the query.
     *
     * @param  mixed  $value
     * @param  "select"|"from"|"join"|"where"|"groupBy"|"having"|"order"|"union"|"unionOrder"  $type
     * @return $this
     *
     * @throws \InvalidArgumentException
     */
    public function addBinding($value, $type = 'where')
    {}

    /**
     * Cast the given binding value.
     *
     * @param  mixed  $value
     * @return mixed
     */
    public function castBinding($value)
    {}

    /**
     * Merge an array of bindings into our bindings.
     *
     * @param  self  $query
     * @return $this
     */
    public function mergeBindings(self $query)
    {}

    /**
     * Remove all of the expressions from a list of bindings.
     *
     * @param  array<mixed>  $bindings
     * @return list<mixed>
     */
    public function cleanBindings(array $bindings)
    {}

    /**
     * Get a scalar type value from an unknown type of input.
     *
     * @param  mixed  $value
     * @return mixed
     */
    protected function flattenValue($value)
    {}

    /**
     * Get the default key name of the table.
     *
     * @return string
     */
    protected function defaultKeyName()
    {}

    /**
     * Get the database connection instance.
     *
     * @return \Illuminate\Database\ConnectionInterface
     */
    public function getConnection()
    {}

    /**
     * Ensure the database connection supports vector queries.
     *
     * @return void
     *
     * @throws \RuntimeException
     */
    protected function ensureConnectionSupportsVectors()
    {}

    /**
     * Get the database query processor instance.
     *
     * @return \Illuminate\Database\Query\Processors\Processor
     */
    public function getProcessor()
    {}

    /**
     * Get the query grammar instance.
     *
     * @return \Illuminate\Database\Query\Grammars\Grammar
     */
    public function getGrammar()
    {}

    /**
     * Use the "write" PDO connection when executing the query.
     *
     * @return $this
     */
    public function useWritePdo()
    {}

    /**
     * Specify arguments for the PDOStatement::fetchAll / fetch functions.
     *
     * @param  mixed  ...$fetchUsing
     * @return $this
     */
    public function fetchUsing(...$fetchUsing)
    {}

    /**
     * Determine if the value is a query builder instance or a Closure.
     *
     * @param  mixed  $value
     * @return bool
     */
    protected function isQueryable($value)
    {}

    /**
     * Clone the query.
     *
     * @return static
     */
    public function clone()
    {}

    /**
     * Clone the query without the given properties.
     *
     * @return static
     */
    public function cloneWithout(array $properties)
    {}

    /**
     * Clone the query without the given bindings.
     *
     * @return static
     */
    public function cloneWithoutBindings(array $except)
    {}

    /**
     * Dump the current SQL and bindings.
     *
     * @param  mixed  ...$args
     * @return $this
     */
    public function dump(...$args)
    {}

    /**
     * Dump the raw current SQL with embedded bindings.
     *
     * @return $this
     */
    public function dumpRawSql()
    {}

    /**
     * Die and dump the current SQL and bindings.
     *
     * @return never
     */
    public function dd()
    {}

    /**
     * Die and dump the current SQL with embedded bindings.
     *
     * @return never
     */
    public function ddRawSql()
    {}

    /**
     * Handle dynamic method calls into the method.
     *
     * @param  string  $method
     * @param  array  $parameters
     * @return mixed
     *
     * @throws \BadMethodCallException
     */
    public function __call($method, $parameters)
    {}
}
