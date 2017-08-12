module Main exposing (..)

import Html exposing (..)
import Ports.Vis.Network as Network exposing (Network)


type Msg
    = Network Network.Msg


type alias Model =
    { network : Network
    }


init : ( Model, Cmd Msg )
init =
    let
        network =
            { divId = "mainNetwork"
            , data = { nodes = [], edges = [] }
            , options = Network.defaultOptions
            }
    in
    { network = network
    }
        ! [ Cmd.map Network (Network.initCmd network) ]


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        Network msg ->
            let
                ( networkModel, networkCmd ) =
                    Network.update msg model.network
            in
            { model | network = networkModel }
                ! [ Cmd.map Network networkCmd ]


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.map Network (Network.subscriptions model.network)



---- VIEW ----


view : Model -> Html Msg
view model =
    Network.view model.network



---- PROGRAM ----


main : Program Never Model Msg
main =
    Html.program
        { view = view
        , init = init
        , update = update
        , subscriptions = subscriptions
        }
