<?php

// routes/web.php
Route::get('/user', 'UserController@index')->middleware('auth');
