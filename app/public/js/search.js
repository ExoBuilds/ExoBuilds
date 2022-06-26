function search_champ()
{
    
    list = document.getElementsByClassName("champion-icon");
    value = document.getElementById("search-champ").value.toLowerCase()
    document.getElementById("searchplayer").href = "./profile/" + value
    for (i = 0; list[i]; i++) {
        if (list[i].id.toLowerCase().includes(value))
            list[i].hidden = false;
        else
            list[i].hidden = true;
    }
}