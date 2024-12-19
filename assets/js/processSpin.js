
function numberWithCommas(x) {
    return x.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',');
}

function toggleLoaders(node){
    var classesString = node.className;
    if(classesString == "") return
    var classes = classesString.split(" ");
    for(var i in classes){  
        node.classList.toggle(classes[i])
    }
}

var update_likes = function (node, id) {
    fetch('/api/v1/count/' + id)
    .then(response => response.json())
    .then(data => {
        console.log("> data ", data)
        console.log("> likes_nodes ", node)
        if (data) {
            node.innerText = numberWithCommas(data.count)
        } else {
            node.innerText = 0
        }
    })
    .catch(error => {
      console.log("Error getting document:", error);
    });
    toggleLoaders(node)
}


var process = function() {
    var likes_nodes = document.querySelectorAll("span[id^='likes_']")

    for (var i in likes_nodes) {
        var node = likes_nodes[i]
        var id = node.id ? node.id.replaceAll("/", "-") : node.id
        if (id) {
            update_likes(node, id)
        }
    }
}()
