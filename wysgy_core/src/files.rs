pub const GV_TEMPLATE_CONTENTS: &str = "
digraph {
        ratio = \"0.9\"; 
        layout=\"dot\";
        splines=\"true\";
        rankdir=\"TB\";
        overlap=\"false\";
        {% set colorscheme = config.colorscheme %}
        {% set fontsize_node = 40 %}
        {% set fontsize_edge = 40 %}
        {% set fontsize_desc = 40 %}

        subgraph cluster_main {rankdir=\"LR\";
                color=\"white\";
                {% for n in nodes %}
                {% if not config.node[n.n.type] %}
                {% set shape = \"box\" %}
                {% set color = \"1\" %}
                {% else %}
                {% set shape = config.node[n.n.type].shape %}
                {% set color = config.node[n.n.type].color %}
                {% endif %}
                {% if n.n.name %}
                {% set label = n.n.name | safe | wordwrap(15, wrapstring=\"<br/>\", break_long_words=False) %}
                {% else %}
                {% set label = n.id %}
                {% endif %}
                {% if n.n.desc %}
                {% set desc = n.n.desc|safe|wordwrap(20, wrapstring=\"<br/>\") %}
                {% else %} 
                {% set desc = \" \" %}
                {% endif %}
                {{ n.id }}[label=< <font point-size='{{ fontsize_node }}'> <B><i> {{ loop.index }}. </i>{{ label }}</B></font> <br/> <font point-size='{{ fontsize_desc }}'>{{ desc }}</font> <br/>  > shape=\"{{ shape }}\" fillcolor=\"{{ color }}\" style=\"filled, rounded\" colorscheme=\"{{ colorscheme }}\"];
                {% endfor %}
        }{% for e in rels %}
        {% if e.desc %}
        {% set reldesc = e.desc|safe|wordwrap(10, wrapstring=\"<br/>\") %}
        {% else %}
        {% set reldesc = \" \" %}
        {% endif %}
        {% if e.name %}
        {% set reltext = e.name | safe | wordwrap(10, wrapstring=\"<br/>\") %}
        {% else %}
        {% set reltext = \" \" %}
        {% endif %}
        {% if config.edge[e.type] %}
        {% set ecolor = config.edge[e.type].color %}
        {% set estyle = config.edge[e.type].style %}
        {% endif %}
        {{ e.src }} -> {{ e.dst }}[label=< <font point-size='{{ fontsize_edge }}'> <i> {{ loop.index }}. </i> {{ reltext }} <br/> <b>{{ reldesc }}</b> </font> > layer=\"{{ e.layer }}\" color=\" {{ ecolor }}\" style=\"{{ estyle }}\" penwidth=\"{{ e.width }}\"];
        {% endfor %}
}
";

pub const CONFIG_JSON_CONTENTS: &str = "
{
  \"colorscheme\": \"bupu9\",
  \"node\": {
    \"intent\": {
      \"color\": 5,
      \"shape\": \"tab\"
    },
    \"event\": {
      \"color\": 1,
      \"shape\": \"doublecircle\"
    },
    \"audio\": {
      \"color\": 2,
      \"shape\": \"doublecircle\"
    },
    \"server\": {
      \"color\": 3,
      \"shape\": \"box\"
    },
    \"ios\": {
      \"color\": 4,
      \"shape\": \"cds\"
    },
    \"condition\": {
      \"color\": 3,
      \"shape\": \"diamond\"
    }
  },
  \"edge\": {
    \"condno\": {
      \"style\": \"dashed\"
    }
  }
}
";
