<?php

namespace Illuminate\Pagination;

use Illuminate\Contracts\Pagination\LengthAwarePaginator as PaginatorContract;

class UrlWindow
{
    /**
     * The paginator implementation.
     *
     * @var \Illuminate\Contracts\Pagination\LengthAwarePaginator
     */
    protected $paginator;

    /**
     * Create a new URL window instance.
     *
     * @param  \Illuminate\Contracts\Pagination\LengthAwarePaginator  $paginator
     */
    public function __construct(PaginatorContract $paginator)
    {}

    /**
     * Create a new URL window instance.
     *
     * @param  \Illuminate\Contracts\Pagination\LengthAwarePaginator  $paginator
     * @return array
     */
    public static function make(PaginatorContract $paginator)
    {}

    /**
     * Get the window of URLs to be shown.
     *
     * @return array
     */
    public function get()
    {}

    /**
     * Get the slider of URLs there are not enough pages to slide.
     *
     * @return array
     */
    protected function getSmallSlider()
    {}

    /**
     * Create a URL slider links.
     *
     * @param  int  $onEachSide
     * @return array
     */
    protected function getUrlSlider($onEachSide)
    {}

    /**
     * Get the slider of URLs when too close to the beginning of the window.
     *
     * @param  int  $window
     * @param  int  $onEachSide
     * @return array
     */
    protected function getSliderTooCloseToBeginning($window, $onEachSide)
    {}

    /**
     * Get the slider of URLs when too close to the ending of the window.
     *
     * @param  int  $window
     * @param  int  $onEachSide
     * @return array
     */
    protected function getSliderTooCloseToEnding($window, $onEachSide)
    {}

    /**
     * Get the slider of URLs when a full slider can be made.
     *
     * @param  int  $onEachSide
     * @return array
     */
    protected function getFullSlider($onEachSide)
    {}

    /**
     * Get the page range for the current page window.
     *
     * @param  int  $onEachSide
     * @return array
     */
    public function getAdjacentUrlRange($onEachSide)
    {}

    /**
     * Get the starting URLs of a pagination slider.
     *
     * @return array
     */
    public function getStart()
    {}

    /**
     * Get the ending URLs of a pagination slider.
     *
     * @return array
     */
    public function getFinish()
    {}

    /**
     * Determine if the underlying paginator being presented has pages to show.
     *
     * @return bool
     */
    public function hasPages()
    {}

    /**
     * Get the current page from the paginator.
     *
     * @return int
     */
    protected function currentPage()
    {}

    /**
     * Get the last page from the paginator.
     *
     * @return int
     */
    protected function lastPage()
    {}
}
