<?php

namespace DeepCopy\TypeFilter\Date;

use DatePeriod;
use DeepCopy\TypeFilter\TypeFilter;

/**
 * @final
 */
class DatePeriodFilter implements TypeFilter
{
    /**
     * {@inheritdoc}
     *
     * @param DatePeriod $element
     *
     * @see http://news.php.net/php.bugs/205076
     */
    public function apply($element)
    {}
}
