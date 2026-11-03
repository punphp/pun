<?php

namespace Illuminate\Database\Concerns;

use Illuminate\Support\Arr;
use Illuminate\Support\Carbon;

trait BuildsWhereDateClauses
{
    /**
     * Add a where clause to determine if a "date" column is in the past to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function wherePast($columns)
    {}

    /**
     * Add a where clause to determine if a "date" column is in the past or now to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function whereNowOrPast($columns)
    {}

    /**
     * Add an "or where" clause to determine if a "date" column is in the past to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function orWherePast($columns)
    {}

    /**
     * Add a where clause to determine if a "date" column is in the past or now to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function orWhereNowOrPast($columns)
    {}

    /**
     * Add a where clause to determine if a "date" column is in the future to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function whereFuture($columns)
    {}

    /**
     * Add a where clause to determine if a "date" column is in the future or now to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function whereNowOrFuture($columns)
    {}

    /**
     * Add an "or where" clause to determine if a "date" column is in the future to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function orWhereFuture($columns)
    {}

    /**
     * Add an "or where" clause to determine if a "date" column is in the future or now to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function orWhereNowOrFuture($columns)
    {}

    /**
     * Add an "where" clause to determine if a "date" column is in the past or future.
     *
     * @param  array|string  $columns
     * @param  string  $operator
     * @param  string  $boolean
     * @return $this
     */
    protected function wherePastOrFuture($columns, $operator, $boolean)
    {}

    /**
     * Add a "where date" clause to determine if a "date" column is today to the query.
     *
     * @param  array|string  $columns
     * @param  string  $boolean
     * @return $this
     */
    public function whereToday($columns, $boolean = 'and')
    {}

    /**
     * Add a "where date" clause to determine if a "date" column is before today.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function whereBeforeToday($columns)
    {}

    /**
     * Add a "where date" clause to determine if a "date" column is today or before to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function whereTodayOrBefore($columns)
    {}

    /**
     * Add a "where date" clause to determine if a "date" column is after today.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function whereAfterToday($columns)
    {}

    /**
     * Add a "where date" clause to determine if a "date" column is today or after to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function whereTodayOrAfter($columns)
    {}

    /**
     * Add an "or where date" clause to determine if a "date" column is today to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function orWhereToday($columns)
    {}

    /**
     * Add an "or where date" clause to determine if a "date" column is before today.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function orWhereBeforeToday($columns)
    {}

    /**
     * Add an "or where date" clause to determine if a "date" column is today or before to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function orWhereTodayOrBefore($columns)
    {}

    /**
     * Add an "or where date" clause to determine if a "date" column is after today.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function orWhereAfterToday($columns)
    {}

    /**
     * Add an "or where date" clause to determine if a "date" column is today or after to the query.
     *
     * @param  array|string  $columns
     * @return $this
     */
    public function orWhereTodayOrAfter($columns)
    {}

    /**
     * Add a "where date" clause to determine if a "date" column is today or after to the query.
     *
     * @param  array|string  $columns
     * @param  string  $operator
     * @param  string  $boolean
     * @return $this
     */
    protected function whereTodayBeforeOrAfter($columns, $operator, $boolean)
    {}
}
