<?php

return view('user.profile')->with([
    'user' => $user,
    'profile' => $profile,
]);
