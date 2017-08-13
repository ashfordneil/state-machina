port module Ports.Vis.Network exposing (..)

import Html exposing (Html, div)
import Html.Attributes exposing (id)


type alias NodeId =
    String


type alias Node =
    { id : NodeId
    , label : String
    , color : String
    }


type alias Edge =
    { from : NodeId
    , to : NodeId
    , label : String
    }


type alias Data =
    { nodes : List Node
    , edges : List Edge
    }


type alias Network =
    { divId : String
    , data : Data
    , options : Options
    }


type Msg
    = StartInit
    | InitSuccessful Bool
    | EdgeSelected Edge
    | UpdateData Data
    | DoubleClick NodeId


initCmd : Network -> Cmd Msg
initCmd =
    initCmdPort


update : Msg -> Network -> ( Network, Cmd Msg )
update msg model =
    case msg of
        InitSuccessful success ->
            model ! []

        UpdateData data ->
            { model | data = data }
                ! [ updateData ( model.divId, data ) ]

        _ ->
            model ! []


subscriptions : Network -> Sub Msg
subscriptions model =
    Sub.batch
        [ initSuccessfulPort InitSuccessful
        , edgeSelected EdgeSelected
        ]


view : Network -> Html msg
view model =
    div [ id model.divId ] []


type alias Options =
    { autoResize : Bool
    , height : String -- height as a css string value eg. "100%"
    , width : String -- same as above
    , locale : LocaleString -- Chosen current locale. 'en', 'es', etc...

    -- if locales is None, defaults by the Vis.js library will be used
    -- , locales : Maybe (Dict LocaleString LocaleSpecificLabels)
    , clickToUse : Bool

    -- , configure : ConfigureOptions
    , edges : EdgesOptions

    -- , nodes : NodesOptions
    -- , groups : GroupsOptions
    , layout : LayoutOptions

    -- , interaction : InteractionOptions
    , manipulation : ManipulationOptions

    -- , physics : PhysicsOptions
    }


defaultOptions : Options
defaultOptions =
    { autoResize = True
    , height = "100%"
    , width = "100%"
    , locale = "en"

    -- , locales = Nothing
    , clickToUse = False

    -- , configure = defaultConfigureOptions
    , edges = defaultEdgesOptions

    -- , nodes = defaultNodesOptions
    -- , groups = defaultGroupsOptions
    , layout = defaultLayoutOptions

    -- , interaction = defaultInteractionOptions
    , manipulation = defaultManipulationOptions

    -- , physics = defaultPhysicsOptions
    }



-- Public, but less-interesting options API


type alias LocaleString =
    String



-- see 'Custom Locales' section at http://visjs.org/docs/network/#locales
-- type alias LocaleSpecificLabels =
--     { edit : String
--     , del : String
--     , back : String
--     , addNode : String
--     , addEdge : String
--     , editNode : String
--     , editEdge : String
--     , addDescription : String
--     , edgeDescription : String
--     , editEdgeDescription : String
--     , createEdgeError : String
--     , deleteClusterError : String
--     , editClusterError : String
--     }
-- type ComponentName
--     = Nodes
--     | Edges
--     | Layout
--     | Interaction
--     | Manipulation
--     | Physics
--     | Selection
--     | Renderer
-- type SubOptions
--     = Edges EdgesOptions
--     | Nodes NodesOptions
--     | Groups GroupsOptions
--     | Layout LayoutOptions
--     | Interaction InteractionOptions
--     | Manipulation ManipulationOptions
--     | Physics PhysicsOptions
-- {-| for more info, please see http://visjs.org/docs/network/configure.html#
-- -}
-- type FilterConfig
--     = AllowAll Bool
--     -- use like `AllowOptions [ Edges, Nodes, Groups ]`
--     | AllowOptions (List (_ -> SubOptions))
--     | OptionPathFilterer (SubOptions -> String -> Bool)
-- type alias ConfigureOptions =
--     { enabled : Bool
--     , filter : FilterConfig
--     -- string == id of DOM object to put the configure list
--     -- if None, default to below the network
--     , container : Maybe String
--     , showButton : Bool
--     }
-- defaultConfigureOptions : ConfigureOptions
-- defaultConfigureOptions =
--     { enabled : True
--     , filter : AllowAll True
--     , container : None
--     , showButton : True
--     }
-- ports


type alias EdgesOptions =
    { arrows : Maybe String
    , font :
        { color : String
        , size : Int
        , face : String
        , strokeWidth : Int
        , strokeColor : String
        , align : String
        }
    }


defaultEdgesOptions : EdgesOptions
defaultEdgesOptions =
    { arrows = Nothing
    , font =
        { color = "#343434"
        , size = 14
        , face = "arial"
        , strokeWidth = 2
        , strokeColor = "#ffffff"
        , align = "horizontal"
        }
    }


type alias LayoutOptions =
    { randomSeed : Maybe Int }


defaultLayoutOptions : LayoutOptions
defaultLayoutOptions =
    { randomSeed = Nothing }


type alias ManipulationOptions =
    { enabled : Bool }


defaultManipulationOptions : ManipulationOptions
defaultManipulationOptions =
    { enabled = False }


port initCmdPort : Network -> Cmd msg


port initSuccessfulPort : (Bool -> msg) -> Sub msg


port edgeSelected : (Edge -> msg) -> Sub msg


port updateData : ( String, Data ) -> Cmd msg
