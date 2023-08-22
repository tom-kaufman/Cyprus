<nav class="navbar">
    <li class="navbar-item">
        <a href="/">Books</a>
    </li>
    <li class="navbar-item">
        <a href="/profile">Profile</a>
    </li>
    <li class="navbar-item">
        <a href="/settings">Settings</a>
    </li>
</nav>

<style>
    .navbar {
        display: flex;
        justify-content: space-around;
        list-style: none;
        padding: 0; 
    }

    .navbar-item {
        flex: 1; 
        text-align: center;
        padding: 10px; 
        background-color: #f8f8f8;
    }

    .navbar-item a {
        display: block; 
        text-decoration: none;
        color: black; 
    }

    .navbar-item:hover {
        background-color: #f0f0f0; 
    }
</style>