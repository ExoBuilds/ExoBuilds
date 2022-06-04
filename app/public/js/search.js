function search_champ()
{
    list = document.getElementsByClassName("champion-icon");
    value = document.getElementById("search-champ").value.toLowerCase()
    for (i = 0; list[i]; i++) {
        if (list[i].id.includes(value))
            list[i].hidden = false;
        else
            list[i].hidden = true;
    }
}