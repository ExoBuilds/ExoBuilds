function construct_champions() {
    fetch("./js/champion.json")
    .then(response => {
        return response.json();
    })
    .then(data => {
        div = document.getElementById("champ-list");
        for (dat in data.data) {
            newA = document.createElement("a");
            newA.id = dat
            newA.classList = "champion-icon"
            newA.href = "http://127.0.0.1:8000/champions/" + dat.toLowerCase()
            newIMG = document.createElement("img");
            newIMG.alt = dat
            newIMG.classList = "champion-image"
            newIMG.src = "http://ddragon.leagueoflegends.com/cdn/12.11.1/img/champion/" + dat + ".png"
            newA.appendChild(newIMG)
            div.appendChild(newA)
        }
    });
}