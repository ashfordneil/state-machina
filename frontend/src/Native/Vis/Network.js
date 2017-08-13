import 'vis/dist/vis-network.min.css'
import { Dataset, Network } from 'vis/index-network';

function getChildNodeByClass(element, className) {
    let foundElement = null, found;
    function recurse(element, className, found) {
        for (let el of element.childNodes) {
            let classes = el.className != undefined? el.className.split(" ") : [];
            for (let className2 of classes) {
                if (className2 == className) {
                    found = true;
                    foundElement = el;
                    break;
                }
            }
            if(found)
                break;
            recurse(el, className, found);
        }
    }
    recurse(element, className, false);
    return foundElement;
}

function editNode(container, data, callback) {
    getChildNodeByClass(container, 'node-label').value = data.label;
    getChildNodeByClass(container, 'node-saveButton').onclick = saveNodeData.bind(this, container, data, callback);
    getChildNodeByClass(container, 'node-cancelButton').onclick = clearNodePopUp.bind(this, container);
    getChildNodeByClass(container, 'node-popUp').style.display = 'block';
}

function editEdgeWithoutDrag(container, data, callback) {
    // filling in the popup DOM elements
    getChildNodeByClass(container, 'edge-label').value = data.label;
    getChildNodeByClass(container, 'edge-saveButton').onclick = saveEdgeData.bind(this, container, data, callback);
    getChildNodeByClass(container, 'edge-cancelButton').onclick = cancelEdgeEdit.bind(this, container, callback);
    getChildNodeByClass(container, 'edge-popUp').style.display = 'block';
}

function clearNodePopUp(container) {
      getChildNodeByClass(container, 'node-saveButton').onclick = null;
      getChildNodeByClass(container, 'node-cancelButton').onclick = null;
      getChildNodeByClass(container, 'node-popUp').style.display = 'none';
}

function cancelNodeEdit(container, callback) {
    clearNodePopUp(container);
    callback(null);
}

function saveNodeData(container, data, callback) {
    data.label = getChildNodeByClass(container, 'node-label').value;
    data.color = "#03A9F4"
    clearNodePopUp(container);
    callback(data);
}

function clearEdgePopUp(container) {
    getChildNodeByClass(container, 'edge-saveButton').onclick = null;
    getChildNodeByClass(container, 'edge-cancelButton').onclick = null;
    getChildNodeByClass(container, 'edge-popUp').style.display = 'none';
}

function cancelEdgeEdit(container, callback) {
    clearEdgePopUp(container);
    callback(null);
}

function saveEdgeData(container, data, callback) {
    if (typeof data.to === 'object')
        data.to = data.to.id
    if (typeof data.from === 'object')
        data.from = data.from.id
    data.label = getChildNodeByClass(container, 'edge-label').value;
    clearEdgePopUp(container);
    callback(data);
}


module.exports = ports => {
    const networkMap = {};

    ports.initCmdPort.subscribe(({ divId, data, options }) => {
        const container = document.getElementById(divId);

        options.manipulation.addNode = (data, callback) => {
            getChildNodeByClass(container, "node-operation").innerHTML = "Add Node"
            editNode(container, data, callback)
        }

        options.manipulation.editNode = (data, callback) => {
            getChildNodeByClass(container, "node-operation").innerHTML = "Edit Node"
            editNode(container, data, callback)
        }

        options.manipulation.addEdge = (data, callback) => {
            getChildNodeByClass(container, "node-operation").innerHTML = "Add Node"
            if (data.from == data.to) {
                let r = confirm("Do you want to connect the node to itself?")
                if (!r) {
                    return callback(null)
                }
            }
            getChildNodeByClass(container, "edge-operation").innerHTML = "Add Edge"
            editEdgeWithoutDrag(container, data, callback)
        }

        options.manipulation.editEdge = {
            editWithoutDrag: (data, callback) => {
                getChildNodeByClass(container, "edge-operation").innerHTML = "Edit Edge"
                editEdgeWithoutDrag(container, data, callback)
            }
        }
        networkMap[divId] = new Network(
            getChildNodeByClass(container, "canvas-container"),
            data,
            options
        );

        ports.initSuccessfulPort.send(true)
    })

    ports.updateData.subscribe(([divId, data]) => {
        console.log(data);
        for (var i = 0; i < data.edges.length; ++i) {
            const { from, to } = data.edges[i];
            data.edges[i].id = JSON.stringify({ from, to });
        }
        networkMap[divId].setData(data);
    });
};
