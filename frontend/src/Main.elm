module Main exposing (..)

import Array
import Dict exposing (Dict)
import Errata
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (onClick)
import Http
import Json.Decode as Decode
import Json.Encode as Encode
import List.Extra
import Ports.Vis.Network as Network exposing (Msg(..), Network, NodeId)
import Set
import Unwrap


type Msg
    = Network Network.Msg
    | ConvertToDFA
    | ConvertToDFAResult (Result Http.Error FA)


type alias Model =
    { currentFA : FA
    , network : Network
    , loading : Bool
    }


type alias InputToken =
    String


{-| Non-Deterministic Finite Automatata
-}
type alias FA =
    { start : NodeId
    , alphabet : List InputToken
    , nodes : Dict NodeId (Dict InputToken (List NodeId))
    , final_states : List NodeId
    }


init : ( Model, Cmd Msg )
init =
    let
        startColor =
            "#4BAE4F"

        normalColor =
            "#03A9F4"

        finishColor =
            "#F34236"

        currentFA =
            { start = "1"
            , alphabet = [ "a", "b" ]
            , nodes =
                Dict.fromList
                    [ ( "1"
                      , Dict.fromList
                            [ ( "a", [ "1", "2" ] )
                            , ( "b", [ "1" ] )
                            ]
                      )
                    , ( "2"
                      , Dict.fromList
                            [ ( "a", [ "3" ] )
                            , ( "b", [ "3" ] )
                            ]
                      )
                    , ( "3", Dict.empty )
                    ]
            , final_states = [ "3" ]
            }

        network =
            { divId = "mainNetwork"
            , data = fa2networkData currentFA
            , options =
                let
                    defaultOptions =
                        Network.defaultOptions

                    defaultEdgesOptions =
                        defaultOptions.edges

                    defaultLayoutOptions =
                        defaultOptions.layout

                    defaultManipulationOptions =
                        defaultOptions.manipulation
                in
                { defaultOptions
                    | height = "700px"
                    , edges = { defaultEdgesOptions | arrows = Just "to" }
                    , layout = { defaultLayoutOptions | randomSeed = Just 0 }
                    , manipulation = { defaultManipulationOptions | enabled = True }
                }
            }
    in
    { network = network
    , currentFA = currentFA
    , loading = False
    }
        ! [ Cmd.map Network (Network.initCmd network) ]


update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        ConvertToDFA ->
            { model | loading = True }
                ! [ Http.post "/submit" (Http.jsonBody (encodeFA model.currentFA)) faDecoder
                        |> Http.send ConvertToDFAResult
                  ]

        ConvertToDFAResult result ->
            let
                fa =
                    Unwrap.result result

                ( networkModel, networkCmd ) =
                    Network.update
                        (fa |> fa2networkData |> Network.UpdateData)
                        model.network
            in
            { model
                | currentFA = fa
                , network = networkModel
            }
                ! [ Cmd.map Network networkCmd ]

        Network msg ->
            let
                currentFA =
                    case msg of
                        DataChanged data ->
                            networkData2fa data

                        _ ->
                            model.currentFA

                ( networkModel, networkCmd ) =
                    Network.update msg model.network
            in
            { model | network = networkModel, currentFA = currentFA }
                ! [ Cmd.map Network networkCmd ]


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.map Network (Network.subscriptions model.network)



---- VIEW ----


view : Model -> Html Msg
view model =
    div [ class "container" ]
        [ h1 [] [ text "State Machina" ]
        , Network.view model.network
        , div [ id "buttons" ]
            [ button [ onClick ConvertToDFA, class "button" ] [ text "OPTIMISE" ] ]
        , Errata.stateMachines
        ]



---- PROGRAM ----


main : Program Never Model Msg
main =
    Html.program
        { view = view
        , init = init
        , update = update
        , subscriptions = subscriptions
        }


networkData2fa : Network.Data -> FA
networkData2fa network =
    let
        alphabet =
            networkData2Alphabet network

        start =
            networkData2StartNode network

        nodes =
            networkData2Nodes network

        final_states =
            networkData2FinalStates network
    in
    { alphabet = alphabet
    , start = start
    , nodes = nodes
    , final_states = final_states
    }


networkData2Alphabet : Network.Data -> List InputToken
networkData2Alphabet network =
    network.edges
        |> List.map (\x -> x.label)
        |> List.concatMap (String.split "/")
        |> Set.fromList
        |> Set.toList


networkData2StartNode : Network.Data -> NodeId
networkData2StartNode network =
    network.nodes
        |> List.filter (\x -> x.color == "#4BAE4F")
        |> List.map (\x -> x.id)
        |> List.head
        |> Unwrap.maybe


createOrAppend : a -> Maybe (List a) -> Maybe (List a)
createOrAppend x xs =
    Just (Maybe.withDefault [] xs ++ [ x ])


networkData2Nodes : Network.Data -> Dict NodeId (Dict InputToken (List NodeId))
networkData2Nodes network =
    let
        initialState =
            network.nodes
                |> List.map (\x -> ( x.id, Dict.empty ))
                |> Dict.fromList
    in
    network.edges
        |> List.concatMap
            (\x ->
                String.split "/" x.label
                    |> List.map (\y -> { from = x.from, to = x.to, letter = y })
            )
        |> List.foldl
            -- a -> b -> b
            (\edge maps ->
                Dict.update edge.from
                    (\transforms ->
                        transforms
                            |> Maybe.withDefault Dict.empty
                            |> Dict.update edge.letter (createOrAppend edge.to)
                            |> Just
                    )
                    maps
            )
            -- b
            initialState


networkData2FinalStates : Network.Data -> List NodeId
networkData2FinalStates network =
    network.nodes
        |> List.filter (\x -> x.color == "#F34236")
        |> List.map (\x -> x.id)
        |> Set.fromList
        |> Set.toList


fa2networkData : FA -> Network.Data
fa2networkData fa =
    let
        edges =
            fa2networkEdges fa

        nodes =
            fa2networkNodes fa
    in
    { nodes = nodes, edges = edges }


fa2networkEdges : FA -> List Network.Edge
fa2networkEdges fa =
    fa.nodes
        |> Dict.toList
        |> List.map
            (\( fromId, inputToken2connections ) ->
                inputToken2connections
                    |> Dict.toList
                    |> List.map
                        (\( inputToken, connections ) ->
                            connections
                                |> List.map
                                    (\toId ->
                                        { from = fromId, to = toId, label = inputToken }
                                    )
                        )
                    |> List.concat
            )
        |> List.concat
        |> List.foldl
            (\edge ->
                \list ->
                    let
                        maybeEdgeAlongSamePathIdx =
                            List.Extra.findIndex
                                (\otherEdge ->
                                    edge.from
                                        == otherEdge.from
                                        && edge.to
                                        == otherEdge.to
                                )
                                list
                    in
                    case maybeEdgeAlongSamePathIdx of
                        Just samePathIdx ->
                            let
                                combinedEdge =
                                    list
                                        |> List.Extra.getAt samePathIdx
                                        |> Unwrap.maybe
                                        |> (\samePathEdge ->
                                                { samePathEdge | label = samePathEdge.label ++ "/" ++ edge.label }
                                           )
                            in
                            list
                                |> Array.fromList
                                |> Array.set samePathIdx combinedEdge
                                |> Array.toList

                        Nothing ->
                            edge :: list
            )
            []


fa2networkNodes : FA -> List Network.Node
fa2networkNodes fa =
    let
        startColor =
            "#4BAE4F"

        normalColor =
            "#03A9F4"

        finishColor =
            "#F34236"
    in
    fa.nodes
        |> Dict.keys
        |> List.map
            (\nodeId ->
                { id = nodeId
                , label = nodeId
                , color =
                    if List.member nodeId fa.final_states then
                        finishColor
                    else if nodeId == fa.start then
                        startColor
                    else
                        normalColor
                }
            )


encodeFA : FA -> Encode.Value
encodeFA fa =
    let
        encodeStrList =
            List.map Encode.string
                >> Encode.list
    in
    [ ( "start", Encode.string fa.start )
    , ( "alphabet", fa.alphabet |> encodeStrList )
    , ( "final_states", fa.final_states |> encodeStrList )
    , ( "nodes"
      , fa.nodes
            |> Dict.toList
            |> List.map
                (\( nodeId, inputToken2connections ) ->
                    ( nodeId
                    , inputToken2connections
                        |> Dict.toList
                        |> List.map (Tuple.mapSecond encodeStrList)
                        |> Encode.object
                    )
                )
            |> Encode.object
      )
    ]
        |> Encode.object


faDecoder : Decode.Decoder FA
faDecoder =
    Decode.map4
        FA
        (Decode.field "start" Decode.string)
        (Decode.field "alphabet" (Decode.list Decode.string))
        (Decode.field "nodes" (Decode.dict (Decode.dict (Decode.string |> Decode.map List.singleton))))
        (Decode.field "final_states" (Decode.list Decode.string))
